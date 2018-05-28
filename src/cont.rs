use elma::Time;
use failure::Error;
use serde_json;

use model::Model;
use shared::{SortBy, SortOrder, WR};
use templ;
use templ::Row;

#[allow(dead_code)]
pub fn get_tt_update_data(model: &Model) -> Result<serde_json::Value, Error> {
	let target_tts = model.get_target_tts();
	let pr_tt = model.get_pr_tt();
	let wr_tts: Vec<_> = model.get_wr_tts().iter().map(|x| x.0).collect();
	Ok(json!({
		"pr_tt": pr_tt.0,
		"target_tts": {
		"godlike":target_tts.godlike.0,
		"legendary": target_tts.legendary.0,
		"world_class": target_tts.world_class.0,
		"professional": target_tts.professional.0,
		"good": target_tts.good.0,
		"ok": target_tts.ok.0,
		"beginner": target_tts.beginner.0},
		"wr_tts": serde_json::to_value(wr_tts)?
		}))
}
pub fn get_level_update_data(model: &Model, level: i32) -> Result<serde_json::Value, Error> {
	let targets = model.get_targets((level - 1) as usize);
	let wrs_for_lev = model.collect_wrs_for_lev((level - 1) as usize);
	let pr_for_lev = model.get_pr((level - 1) as usize);
	Ok(json!({"level": level,
		"times": serde_json::to_value(wrs_for_lev)?,
		"pr": pr_for_lev.0,
		"targets": {
			"godlike":targets.godlike.0,
			"legendary": targets.legendary.0,
			"world_class": targets.world_class.0,
			"professional": targets.professional.0,
			"good": targets.good.0,
			"ok": targets.ok.0,
			"beginner": targets.beginner.0}
		}))
}

pub fn build_table_update_data(model: &Model, sort_by: SortBy) -> Result<serde_json::Value, Error> {
	let mut data = populate_table_data(&model);

	// * Footer
	let (p_tt, target_wr_tt) = compute_tts(&data);
	let target_tt = model.get_next_targets_tt();

	let p_tt_class = model.get_tt_class(p_tt);
	let target_tt_class = model.get_tt_class(target_tt);
	let target_wr_tt_class = model.get_tt_class(target_wr_tt);

	let footer = templ::table_footer(
		p_tt,
		&p_tt_class,
		target_wr_tt,
		&target_wr_tt_class,
		target_tt,
		&target_tt_class,
	);

	// * Body
	sort_table_data(&mut data, &model, sort_by)?;
	let table_rows = templ::table_body(&data);

	Ok(json!({"rows": table_rows, "footer": footer}))
}

pub fn populate_table_data(model: &Model) -> Vec<Row> {
	let level_names = vec![
		"Warm Up",
		"Flat Track",
		"Twin Peaks",
		"Over and Under",
		"Uphill Battle",
		"Long Haul",
		"Hi Flyer",
		"Tag",
		"Tunnel Terror",
		"The Steppes",
		"Gravity Ride",
		"Islands in the Sky",
		"Hill Legend",
		"Loop-de-Loop",
		"Serpents Tale",
		"New Wave",
		"Labyrinth",
		"Spiral",
		"Turnaround",
		"Upside Down",
		"Hangman",
		"Slalom",
		"Quick Round",
		"Ramp Frenzy",
		"Precarious",
		"Circuitous",
		"Shelf Life",
		"Bounce Back",
		"Headbanger",
		"Pipe",
		"Animal Farm",
		"Steep Corner",
		"Zig-Zag",
		"Bumpy Journey",
		"Labyrinth Pro",
		"Fruit in the Den",
		"Jaws",
		"Curvaceous",
		"Haircut",
		"Double Trouble",
		"Framework",
		"Enduro",
		"He He",
		"Freefall",
		"Sink",
		"Bowling",
		"Enigma",
		"Downhill",
		"What the Heck",
		"Expert System",
		"Tricks Abound",
		"Hang Tight",
		"Hooked",
		"Apple Harvest",
	];

	level_names
		.iter()
		.enumerate()
		.map(|(i, lev_name)| {
			let lev_number = i as i32 + 1;
			let pr = model.get_pr(i);
			let pr_class = model.get_time_class(&pr, i);

			// ! deal with options
			let last_wr_beat = model.get_last_wr_beat(&pr, i);
			let (_, kuski_beat, kuski_beat_table, wr_beat) = wr_to_values(&last_wr_beat);
			let wr_beat_class = model.get_time_class(&wr_beat, i);

			let first_wr_not_beat = model.get_first_wr_not_beat(&pr, i);
			let (_, kuski_not_beat, kuski_not_beat_table, wr_not_beat) =
				wr_to_values(&first_wr_not_beat);
			let wr_not_beat_class = model.get_time_class(&wr_not_beat, i);

			let target = model.get_next_target(&pr, i);
			let target_class = model.get_time_class(&target, i);

			Row {
				lev_number,
				lev_name: lev_name.to_string(),
				pr,
				pr_class,
				kuski_beat,
				kuski_beat_table,
				wr_beat: wr_beat,
				wr_beat_class,
				kuski_not_beat,
				kuski_not_beat_table,
				wr_not_beat: wr_not_beat,
				wr_not_beat_class,
				target,
				target_class,
			}
		})
		.collect()
}

pub fn wr_to_values(wr: &Option<WR>) -> (i32, String, i32, Time) {
	if let Some(WR {
		table,
		lev,
		time,
		ref kuski,
	}) = *wr
	{
		(lev, kuski.to_string(), table, time)
	} else {
		(0, "".to_owned(), 0, Time(0))
	}
}

fn compute_tts(rs: &[Row]) -> (Time, Time) {
	rs.iter().fold((Time(0), Time(0)), |acc, r| {
		(
			acc.0 + r.pr,
			acc.1 + if r.wr_not_beat != Time(0) {
				r.wr_not_beat
			} else {
				r.pr
			},
		)
	})
}

fn sort_table_data(data: &mut Vec<Row>, model: &Model, sort_by: SortBy) -> Result<(), Error> {
	match sort_by {
		SortBy::Table(ord) => data.sort_by(|x, y| {
			let table1: i32 = x.kuski_beat_table;
			let table2: i32 = y.kuski_beat_table;
			match ord {
				SortOrder::Ascending => table1.cmp(&table2),
				SortOrder::Descending => table2.cmp(&table1),
			}
		}),
		SortBy::PR(ord) => data.sort_by(|x, y| {
			let pr1 = x.pr;
			let pr2 = y.pr;
			match ord {
				SortOrder::Ascending => pr1.cmp(&pr2),
				SortOrder::Descending => pr2.cmp(&pr1),
			}
		}),
		SortBy::DiffToNextTarget(ord) => data.sort_by(|x, y| {
			let pr1 = x.pr;
			let lev_num1 = (x.lev_number - 1) as usize;
			let tar1 = model.get_next_target(&pr1, lev_num1);
			let pr2 = y.pr;
			let lev_num2 = (y.lev_number - 1) as usize;
			let tar2 = model.get_next_target(&pr2, lev_num2);
			match ord {
				SortOrder::Ascending => (pr1 - tar1).cmp(&(pr2 - tar2)),
				SortOrder::Descending => (pr2 - tar2).cmp(&(pr1 - tar1)),
			}
		}),
		SortBy::DiffToPrevWR(ord) => data.sort_by(|x, y| {
			let pr1 = x.pr;
			let wr1 = x.wr_beat;
			let pr2 = y.pr;
			let wr2 = y.wr_beat;
			match ord {
				SortOrder::Ascending => (pr1 - wr1).cmp(&(pr2 - wr2)),
				SortOrder::Descending => (pr2 - wr2).cmp(&(pr1 - wr1)),
			}
		}),
		SortBy::DiffToNextWR(ord) => data.sort_by(|x, y| {
			let pr1 = x.pr;
			let wr1 = x.wr_not_beat;
			let pr2 = y.pr;
			let wr2 = y.wr_not_beat;
			match ord {
				SortOrder::Ascending => (pr1 - wr1).cmp(&(pr2 - wr2)),
				SortOrder::Descending => (pr2 - wr2).cmp(&(pr1 - wr1)),
			}
		}),
		SortBy::LevelNum(ord) => match ord {
			SortOrder::Ascending => {}
			SortOrder::Descending => data.sort_by(|x, y| {
				let lev_num1 = x.lev_number;
				let lev_num2 = y.lev_number;
				lev_num2.cmp(&lev_num1)
			}),
		},
	}
	Ok(())
}
