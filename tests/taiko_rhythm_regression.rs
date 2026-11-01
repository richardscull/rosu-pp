use rosu_pp::{Beatmap, Difficulty, taiko::TaikoPerformance};

#[test]
fn taiko_matches_osu_reference() {
    // osu! PerformanceCalculator 2026.702.1.
    for (path, mods, lazer, greats, stars, rhythm, pp) in [
        (
            "resources/1028484.osu",
            8,
            false,
            277,
            2.9144215641617333,
            0.15668593055086977,
            92.66767874287677,
        ),
        (
            "resources/2785319.osu",
            256,
            false,
            896,
            3.513888652564866,
            0.4475307233879461,
            173.34932252231278,
        ),
        (
            "resources/1028484.osu",
            16,
            true,
            277,
            2.9957310404617963,
            0.134624894028623,
            115.23150188074916,
        ),
        (
            "resources/1028484.osu",
            64,
            true,
            277,
            4.051329399964723,
            0.570672991636439,
            204.47009318594485,
        ),
        (
            "resources/100.osu",
            0,
            true,
            77,
            0.9103396463122263,
            0.05084456589562116,
            24.016713301274066,
        ),
        (
            "resources/1005957.osu",
            8,
            true,
            2287,
            2.5366735050225526,
            0.2859931833599458,
            138.86274665202598,
        ),
        (
            "resources/1044694.osu",
            64,
            true,
            1680,
            6.542952101858333,
            1.1742683398541762,
            489.64495033801904,
        ),
        (
            "resources/1138626.osu",
            256,
            true,
            217,
            0.9702453985400253,
            0.008707202518703803,
            9.477834384360696,
        ),
        (
            "resources/1000747.osu",
            16,
            true,
            2671,
            4.729094617175021,
            0.7100039315375998,
            405.87995217808736,
        ),
        (
            "resources/2785319.osu",
            10,
            true,
            896,
            4.475344582059233,
            0.6076913406277198,
            209.87256646226774,
        ),
    ] {
        let map = Beatmap::from_path(path).unwrap();
        let attrs = TaikoPerformance::new(&map)
            .difficulty(Difficulty::new().mods(mods).lazer(lazer))
            .n300(greats)
            .n100(10)
            .misses(2)
            .calculate()
            .unwrap();

        assert!(
            (attrs.difficulty.stars - stars).abs() < 1e-8,
            "{path}: stars {} != {stars}",
            attrs.difficulty.stars
        );
        assert!(
            (attrs.difficulty.rhythm - rhythm).abs() < 1e-8,
            "{path}: rhythm {} != {rhythm}",
            attrs.difficulty.rhythm
        );
        assert!(
            (attrs.pp - pp).abs() < 1e-8,
            "{path}: pp {} != {pp}",
            attrs.pp
        );
    }
}
