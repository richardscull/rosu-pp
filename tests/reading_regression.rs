use rosu_pp::{Beatmap, osu::OsuPerformance};

#[test]
fn osu_classic_reading_accounts_for_slider_breaks() {
    let map = Beatmap::from_path("resources/2785319.osu").unwrap();
    let attrs = OsuPerformance::new(&map)
        .mods(8)
        .lazer(false)
        .combo(300)
        .n300(566)
        .n100(20)
        .n50(5)
        .misses(10)
        .legacy_total_score(1_000_000)
        .calculate()
        .unwrap();

    // osu! PerformanceCalculator 2026.702.1, HD+CL, same hit results.
    assert!((attrs.pp_reading - 16.606663041743428).abs() < 1e-9);
    assert!((attrs.pp - 107.5465267082546).abs() < 1e-9);
}
