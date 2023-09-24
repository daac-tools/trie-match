use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use phf::phf_map;
use trie_match::trie_match;

fn load_input(path: impl AsRef<Path>) -> Vec<String> {
    let mut result = vec![];
    let file = BufReader::new(File::open(path.as_ref()).unwrap());
    for line in file.lines() {
        result.push(line.unwrap());
    }
    result
}

fn criterion_word100(c: &mut Criterion) {
    let mut group = c.benchmark_group("words_100");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(5));
    group.sampling_mode(SamplingMode::Flat);

    let word_100 = load_input("benches/input_word_100.txt");

    group.bench_function("phfmap_rand", |b| {
        static STATIC_MAP: phf::Map<&'static str, i32> = phf_map! {
            "stampeding" => 3141,
            "commendable" => 3141,
            "adrenaline" => 3141,
            "exobiology" => 3141,
            "indifference" => 3141,
            "avuncular" => 3141,
            "prevailed" => 3141,
            "foreparts" => 3141,
            "legalistically" => 3141,
            "intermarries" => 3141,
            "desideratum" => 3141,
            "evaluating" => 3141,
            "lavishing" => 3141,
            "attractable" => 3141,
            "philippics" => 3141,
            "antiabortionist" => 3141,
            "lascivious" => 3141,
            "breathable" => 3141,
            "histogram" => 3141,
            "rattlings" => 3141,
            "interdict" => 3141,
            "summarized" => 3141,
            "relieving" => 3141,
            "congresspeople" => 3141,
            "fitfulness" => 3141,
            "percolation" => 5926,
            "upperclasswoman" => 5926,
            "epistemic" => 5926,
            "Chantilly" => 5926,
            "stonemasons" => 5926,
            "nonferrous" => 5926,
            "emulsions" => 5926,
            "charitably" => 5926,
            "barracudas" => 5926,
            "integrity" => 5926,
            "knockdowns" => 5926,
            "roadworks" => 5926,
            "abortionists" => 5926,
            "Salvadoran" => 5926,
            "chanceries" => 5926,
            "misinform" => 5926,
            "caretaker" => 5926,
            "extricated" => 5926,
            "mandolins" => 5926,
            "steeliest" => 5926,
            "transpiration" => 5926,
            "weirdness" => 5926,
            "audiologists" => 5926,
            "baronetcies" => 5926,
            "performing" => 5926,
            "publishing" => 5358,
            "suspending" => 5358,
            "dermatological" => 5358,
            "contemplate" => 5358,
            "spiritless" => 5358,
            "nightwatchman" => 5358,
            "paradisaical" => 5358,
            "implicating" => 5358,
            "timpanists" => 5358,
            "Leavenworth" => 5358,
            "amorality" => 5358,
            "strangulated" => 5358,
            "cellophane" => 5358,
            "waterboard" => 5358,
            "astrophysicists" => 5358,
            "aerospace" => 5358,
            "passphrase" => 5358,
            "engendered" => 5358,
            "spotlighting" => 5358,
            "misapplication" => 5358,
            "barterers" => 5358,
            "poetesses" => 5358,
            "dollhouse" => 5358,
            "laparoscopic" => 5358,
            "Dubrovnik" => 5358,
            "rerecords" => 9793,
            "shielding" => 9793,
            "orthographically" => 9793,
            "thicknesses" => 9793,
            "Bendictus" => 9793,
            "congealed" => 9793,
            "cooperative" => 9793,
            "encompass" => 9793,
            "grouching" => 9793,
            "shipowners" => 9793,
            "jealously" => 9793,
            "generational" => 9793,
            "antecedents" => 9793,
            "persecutes" => 9793,
            "exemplified" => 9793,
            "admirable" => 9793,
            "squeakiest" => 9793,
            "absconding" => 9793,
            "extirpated" => 9793,
            "exoskeletons" => 9793,
            "earthworms" => 9793,
            "chaotically" => 9793,
            "shipbuilder" => 9793,
            "equidistantly" => 9793,
            "overprint" => 9793,
        };
        b.iter(|| {
            let mut x = 0;
            for s in &word_100 {
                x += STATIC_MAP.get(s).unwrap_or(&0);
            }
            x
        });
    });

    group.bench_function("match_rand", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &word_100 {
                match s.as_str() {
                    "stampeding" | "commendable" | "adrenaline" | "exobiology" | "indifference"
                    | "avuncular" | "prevailed" | "foreparts" | "legalistically"
                    | "intermarries" | "desideratum" | "evaluating" | "lavishing"
                    | "attractable" | "philippics" | "antiabortionist" | "lascivious"
                    | "breathable" | "histogram" | "rattlings" | "interdict" | "summarized"
                    | "relieving" | "congresspeople" | "fitfulness" => {
                        x += 3141;
                    }
                    "percolation" | "upperclasswoman" | "epistemic" | "Chantilly"
                    | "stonemasons" | "nonferrous" | "emulsions" | "charitably" | "barracudas"
                    | "integrity" | "knockdowns" | "roadworks" | "abortionists" | "Salvadoran"
                    | "chanceries" | "misinform" | "caretaker" | "extricated" | "mandolins"
                    | "steeliest" | "transpiration" | "weirdness" | "audiologists"
                    | "baronetcies" | "performing" => {
                        x += 5926;
                    }
                    "publishing" | "suspending" | "dermatological" | "contemplate"
                    | "spiritless" | "nightwatchman" | "paradisaical" | "implicating"
                    | "timpanists" | "Leavenworth" | "amorality" | "strangulated"
                    | "cellophane" | "waterboard" | "astrophysicists" | "aerospace"
                    | "passphrase" | "engendered" | "spotlighting" | "misapplication"
                    | "barterers" | "poetesses" | "dollhouse" | "laparoscopic" | "Dubrovnik" => {
                        x += 5358;
                    }
                    "rerecords" | "shielding" | "orthographically" | "thicknesses"
                    | "Bendictus" | "congealed" | "cooperative" | "encompass" | "grouching"
                    | "shipowners" | "jealously" | "generational" | "antecedents"
                    | "persecutes" | "exemplified" | "admirable" | "squeakiest" | "absconding"
                    | "extirpated" | "exoskeletons" | "earthworms" | "chaotically"
                    | "shipbuilder" | "equidistantly" | "overprint" => {
                        x += 9793;
                    }
                    _ => {}
                }
            }
            x
        });
    });

    group.bench_function("match_1", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &word_100 {
                match s.as_str() {
                    "stampeding" | "commendable" | "adrenaline" | "exobiology" | "indifference"
                    | "avuncular" | "prevailed" | "foreparts" | "legalistically"
                    | "intermarries" | "desideratum" | "evaluating" | "lavishing"
                    | "attractable" | "philippics" | "antiabortionist" | "lascivious"
                    | "breathable" | "histogram" | "rattlings" | "interdict" | "summarized"
                    | "relieving" | "congresspeople" | "fitfulness" => {
                        x += 1;
                    }
                    "percolation" | "upperclasswoman" | "epistemic" | "Chantilly"
                    | "stonemasons" | "nonferrous" | "emulsions" | "charitably" | "barracudas"
                    | "integrity" | "knockdowns" | "roadworks" | "abortionists" | "Salvadoran"
                    | "chanceries" | "misinform" | "caretaker" | "extricated" | "mandolins"
                    | "steeliest" | "transpiration" | "weirdness" | "audiologists"
                    | "baronetcies" | "performing" => {
                        x += 2;
                    }
                    "publishing" | "suspending" | "dermatological" | "contemplate"
                    | "spiritless" | "nightwatchman" | "paradisaical" | "implicating"
                    | "timpanists" | "Leavenworth" | "amorality" | "strangulated"
                    | "cellophane" | "waterboard" | "astrophysicists" | "aerospace"
                    | "passphrase" | "engendered" | "spotlighting" | "misapplication"
                    | "barterers" | "poetesses" | "dollhouse" | "laparoscopic" | "Dubrovnik" => {
                        x += 3;
                    }
                    "rerecords" | "shielding" | "orthographically" | "thicknesses"
                    | "Bendictus" | "congealed" | "cooperative" | "encompass" | "grouching"
                    | "shipowners" | "jealously" | "generational" | "antecedents"
                    | "persecutes" | "exemplified" | "admirable" | "squeakiest" | "absconding"
                    | "extirpated" | "exoskeletons" | "earthworms" | "chaotically"
                    | "shipbuilder" | "equidistantly" | "overprint" => {
                        x += 4;
                    }
                    _ => {}
                }
            }
            x
        });
    });

    group.bench_function("match_0", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &word_100 {
                match s.as_str() {
                    "stampeding" | "commendable" | "adrenaline" | "exobiology" | "indifference"
                    | "avuncular" | "prevailed" | "foreparts" | "legalistically"
                    | "intermarries" | "desideratum" | "evaluating" | "lavishing"
                    | "attractable" | "philippics" | "antiabortionist" | "lascivious"
                    | "breathable" | "histogram" | "rattlings" | "interdict" | "summarized"
                    | "relieving" | "congresspeople" | "fitfulness" => {
                        x += 0;
                    }
                    "percolation" | "upperclasswoman" | "epistemic" | "Chantilly"
                    | "stonemasons" | "nonferrous" | "emulsions" | "charitably" | "barracudas"
                    | "integrity" | "knockdowns" | "roadworks" | "abortionists" | "Salvadoran"
                    | "chanceries" | "misinform" | "caretaker" | "extricated" | "mandolins"
                    | "steeliest" | "transpiration" | "weirdness" | "audiologists"
                    | "baronetcies" | "performing" => {
                        x += 1;
                    }
                    "publishing" | "suspending" | "dermatological" | "contemplate"
                    | "spiritless" | "nightwatchman" | "paradisaical" | "implicating"
                    | "timpanists" | "Leavenworth" | "amorality" | "strangulated"
                    | "cellophane" | "waterboard" | "astrophysicists" | "aerospace"
                    | "passphrase" | "engendered" | "spotlighting" | "misapplication"
                    | "barterers" | "poetesses" | "dollhouse" | "laparoscopic" | "Dubrovnik" => {
                        x += 2;
                    }
                    "rerecords" | "shielding" | "orthographically" | "thicknesses"
                    | "Bendictus" | "congealed" | "cooperative" | "encompass" | "grouching"
                    | "shipowners" | "jealously" | "generational" | "antecedents"
                    | "persecutes" | "exemplified" | "admirable" | "squeakiest" | "absconding"
                    | "extirpated" | "exoskeletons" | "earthworms" | "chaotically"
                    | "shipbuilder" | "equidistantly" | "overprint" => {
                        x += 3;
                    }
                    _ => {}
                }
            }
            x
        });
    });

    group.bench_function("trie_match_rand", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &word_100 {
                trie_match!(match s.as_str() {
                    "stampeding" | "commendable" | "adrenaline" | "exobiology" | "indifference"
                    | "avuncular" | "prevailed" | "foreparts" | "legalistically"
                    | "intermarries" | "desideratum" | "evaluating" | "lavishing"
                    | "attractable" | "philippics" | "antiabortionist" | "lascivious"
                    | "breathable" | "histogram" | "rattlings" | "interdict" | "summarized"
                    | "relieving" | "congresspeople" | "fitfulness" => {
                        x += 3141;
                    }
                    "percolation" | "upperclasswoman" | "epistemic" | "Chantilly"
                    | "stonemasons" | "nonferrous" | "emulsions" | "charitably" | "barracudas"
                    | "integrity" | "knockdowns" | "roadworks" | "abortionists" | "Salvadoran"
                    | "chanceries" | "misinform" | "caretaker" | "extricated" | "mandolins"
                    | "steeliest" | "transpiration" | "weirdness" | "audiologists"
                    | "baronetcies" | "performing" => {
                        x += 5926;
                    }
                    "publishing" | "suspending" | "dermatological" | "contemplate"
                    | "spiritless" | "nightwatchman" | "paradisaical" | "implicating"
                    | "timpanists" | "Leavenworth" | "amorality" | "strangulated"
                    | "cellophane" | "waterboard" | "astrophysicists" | "aerospace"
                    | "passphrase" | "engendered" | "spotlighting" | "misapplication"
                    | "barterers" | "poetesses" | "dollhouse" | "laparoscopic" | "Dubrovnik" => {
                        x += 5358;
                    }
                    "rerecords" | "shielding" | "orthographically" | "thicknesses"
                    | "Bendictus" | "congealed" | "cooperative" | "encompass" | "grouching"
                    | "shipowners" | "jealously" | "generational" | "antecedents"
                    | "persecutes" | "exemplified" | "admirable" | "squeakiest" | "absconding"
                    | "extirpated" | "exoskeletons" | "earthworms" | "chaotically"
                    | "shipbuilder" | "equidistantly" | "overprint" => {
                        x += 9793;
                    }
                    _ => {}
                })
            }
            x
        });
    });

    group.bench_function("trie_match_1", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &word_100 {
                trie_match!(match s.as_str() {
                    "stampeding" | "commendable" | "adrenaline" | "exobiology" | "indifference"
                    | "avuncular" | "prevailed" | "foreparts" | "legalistically"
                    | "intermarries" | "desideratum" | "evaluating" | "lavishing"
                    | "attractable" | "philippics" | "antiabortionist" | "lascivious"
                    | "breathable" | "histogram" | "rattlings" | "interdict" | "summarized"
                    | "relieving" | "congresspeople" | "fitfulness" => {
                        x += 1;
                    }
                    "percolation" | "upperclasswoman" | "epistemic" | "Chantilly"
                    | "stonemasons" | "nonferrous" | "emulsions" | "charitably" | "barracudas"
                    | "integrity" | "knockdowns" | "roadworks" | "abortionists" | "Salvadoran"
                    | "chanceries" | "misinform" | "caretaker" | "extricated" | "mandolins"
                    | "steeliest" | "transpiration" | "weirdness" | "audiologists"
                    | "baronetcies" | "performing" => {
                        x += 2;
                    }
                    "publishing" | "suspending" | "dermatological" | "contemplate"
                    | "spiritless" | "nightwatchman" | "paradisaical" | "implicating"
                    | "timpanists" | "Leavenworth" | "amorality" | "strangulated"
                    | "cellophane" | "waterboard" | "astrophysicists" | "aerospace"
                    | "passphrase" | "engendered" | "spotlighting" | "misapplication"
                    | "barterers" | "poetesses" | "dollhouse" | "laparoscopic" | "Dubrovnik" => {
                        x += 3;
                    }
                    "rerecords" | "shielding" | "orthographically" | "thicknesses"
                    | "Bendictus" | "congealed" | "cooperative" | "encompass" | "grouching"
                    | "shipowners" | "jealously" | "generational" | "antecedents"
                    | "persecutes" | "exemplified" | "admirable" | "squeakiest" | "absconding"
                    | "extirpated" | "exoskeletons" | "earthworms" | "chaotically"
                    | "shipbuilder" | "equidistantly" | "overprint" => {
                        x += 4;
                    }
                    _ => {}
                })
            }
            x
        });
    });

    group.bench_function("trie_match_0", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &word_100 {
                trie_match!(match s.as_str() {
                    "stampeding" | "commendable" | "adrenaline" | "exobiology" | "indifference"
                    | "avuncular" | "prevailed" | "foreparts" | "legalistically"
                    | "intermarries" | "desideratum" | "evaluating" | "lavishing"
                    | "attractable" | "philippics" | "antiabortionist" | "lascivious"
                    | "breathable" | "histogram" | "rattlings" | "interdict" | "summarized"
                    | "relieving" | "congresspeople" | "fitfulness" => {
                        x += 0;
                    }
                    "percolation" | "upperclasswoman" | "epistemic" | "Chantilly"
                    | "stonemasons" | "nonferrous" | "emulsions" | "charitably" | "barracudas"
                    | "integrity" | "knockdowns" | "roadworks" | "abortionists" | "Salvadoran"
                    | "chanceries" | "misinform" | "caretaker" | "extricated" | "mandolins"
                    | "steeliest" | "transpiration" | "weirdness" | "audiologists"
                    | "baronetcies" | "performing" => {
                        x += 1;
                    }
                    "publishing" | "suspending" | "dermatological" | "contemplate"
                    | "spiritless" | "nightwatchman" | "paradisaical" | "implicating"
                    | "timpanists" | "Leavenworth" | "amorality" | "strangulated"
                    | "cellophane" | "waterboard" | "astrophysicists" | "aerospace"
                    | "passphrase" | "engendered" | "spotlighting" | "misapplication"
                    | "barterers" | "poetesses" | "dollhouse" | "laparoscopic" | "Dubrovnik" => {
                        x += 2;
                    }
                    "rerecords" | "shielding" | "orthographically" | "thicknesses"
                    | "Bendictus" | "congealed" | "cooperative" | "encompass" | "grouching"
                    | "shipowners" | "jealously" | "generational" | "antecedents"
                    | "persecutes" | "exemplified" | "admirable" | "squeakiest" | "absconding"
                    | "extirpated" | "exoskeletons" | "earthworms" | "chaotically"
                    | "shipbuilder" | "equidistantly" | "overprint" => {
                        x += 3;
                    }
                    _ => {}
                })
            }
            x
        });
    });
}

fn criterion_html_elements(c: &mut Criterion) {
    let mut group = c.benchmark_group("html_elements");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(5));
    group.sampling_mode(SamplingMode::Flat);

    let html_elements = load_input("benches/input_html_elements.txt");

    group.bench_function("phfmap_rand", |b| {
        static STATIC_MAP: phf::Map<&'static str, i32> = phf_map! {
            "bdo" => 3141,
            "rb" => 3141,
            "th" => 3141,
            "ul" => 3141,
            "pre" => 3141,
            "mark" => 3141,
            "em" => 3141,
            "search" => 3141,
            "head" => 3141,
            "li" => 3141,
            "del" => 3141,
            "details" => 3141,
            "p" => 3141,
            "bdi" => 3141,
            "time" => 3141,
            "area" => 3141,
            "br" => 3141,
            "var" => 3141,
            "aside" => 3141,
            "main" => 3141,
            "tfoot" => 3141,
            "hr" => 3141,
            "label" => 3141,
            "rp" => 3141,
            "menuitem" => 3141,
            "portal" => 5926,
            "wbr" => 5926,
            "cite" => 5926,
            "ins" => 5926,
            "footer" => 5926,
            "table" => 5926,
            "address" => 5926,
            "div" => 5926,
            "optgroup" => 5926,
            "dd" => 5926,
            "samp" => 5926,
            "map" => 5926,
            "xmp" => 5926,
            "embed" => 5926,
            "strong" => 5926,
            "dialog" => 5926,
            "colgroup" => 5926,
            "input" => 5926,
            "figure" => 5926,
            "body" => 5926,
            "strike" => 5926,
            "audio" => 5926,
            "marquee" => 5926,
            "noscript" => 5926,
            "form" => 5926,
            "nobr" => 5358,
            "font" => 5358,
            "textarea" => 5358,
            "tbody" => 5358,
            "picture" => 5358,
            "legend" => 5358,
            "img" => 5358,
            "progress" => 5358,
            "meter" => 5358,
            "script" => 5358,
            "dt" => 5358,
            "summary" => 5358,
            "ol" => 5358,
            "acronym" => 5358,
            "header" => 5358,
            "title" => 5358,
            "span" => 5358,
            "abbr" => 5358,
            "hgroup" => 5358,
            "meta" => 5358,
            "plaintext" => 5358,
            "base" => 5358,
            "sub" => 5358,
            "select" => 5358,
            "s" => 5358,
            "output" => 9793,
            "datalist" => 9793,
            "article" => 9793,
            "param" => 9793,
            "blockquote" => 9793,
            "i" => 9793,
            "tr" => 9793,
            "html" => 9793,
            "section" => 9793,
            "link" => 9793,
            "small" => 9793,
            "canvas" => 9793,
            "option" => 9793,
            "dir" => 9793,
            "col" => 9793,
            "noembed" => 9793,
            "rtc" => 9793,
            "big" => 9793,
            "figcaption" => 9793,
            "kbd" => 9793,
            "b" => 9793,
            "u" => 9793,
            "a" => 9793,
            "td" => 9793,
            "center" => 9793,
            "menu" => 2384,
            "template" => 2384,
            "data" => 2384,
            "image" => 2384,
            "fieldset" => 2384,
            "slot" => 2384,
            "q" => 2384,
            "thead" => 2384,
            "nav" => 2384,
            "style" => 2384,
            "button" => 2384,
            "video" => 2384,
            "dl" => 2384,
            "caption" => 2384,
            "ruby" => 2384,
            "tt" => 2384,
            "dfn" => 2384,
            "code" => 2384,
            "source" => 2384,
            "h1" => 2384,
            "iframe" => 2384,
            "sup" => 2384,
            "noframes" => 2384,
            "frameset" => 2384,
            "track" => 2384,
            "frame" => 2384,
            "rt" => 2384,
            "object"  => 2384,
        };
        b.iter(|| {
            let mut x = 0;
            for s in &html_elements {
                x += STATIC_MAP.get(s).unwrap_or(&0);
            }
            x
        });
    });

    group.bench_function("match_rand", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &html_elements {
                match s.as_str() {
                    "bdo" | "rb" | "th" | "ul" | "pre" | "mark" | "em" | "search" | "head"
                    | "li" | "del" | "details" | "p" | "bdi" | "time" | "area" | "br" | "var"
                    | "aside" | "main" | "tfoot" | "hr" | "label" | "rp" | "menuitem" => {
                        x += 3141;
                    }
                    "portal" | "wbr" | "cite" | "ins" | "footer" | "table" | "address" | "div"
                    | "optgroup" | "dd" | "samp" | "map" | "xmp" | "embed" | "strong"
                    | "dialog" | "colgroup" | "input" | "figure" | "body" | "strike" | "audio"
                    | "marquee" | "noscript" | "form" => {
                        x += 5926;
                    }
                    "nobr" | "font" | "textarea" | "tbody" | "picture" | "legend" | "img"
                    | "progress" | "meter" | "script" | "dt" | "summary" | "ol" | "acronym"
                    | "header" | "title" | "span" | "abbr" | "hgroup" | "meta" | "plaintext"
                    | "base" | "sub" | "select" | "s" => {
                        x += 5358;
                    }
                    "output" | "datalist" | "article" | "param" | "blockquote" | "i" | "tr"
                    | "html" | "section" | "link" | "small" | "canvas" | "option" | "dir"
                    | "col" | "noembed" | "rtc" | "big" | "figcaption" | "kbd" | "b" | "u"
                    | "a" | "td" | "center" => {
                        x += 9793;
                    }
                    "menu" | "template" | "data" | "image" | "fieldset" | "slot" | "q"
                    | "thead" | "nav" | "style" | "button" | "video" | "dl" | "caption"
                    | "ruby" | "tt" | "dfn" | "code" | "source" | "h1" | "iframe" | "sup"
                    | "noframes" | "frameset" | "track" | "frame" | "rt" | "object" => {
                        x += 2384;
                    }
                    _ => {}
                }
            }
            x
        });
    });

    group.bench_function("match_1", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &html_elements {
                match s.as_str() {
                    "bdo" | "rb" | "th" | "ul" | "pre" | "mark" | "em" | "search" | "head"
                    | "li" | "del" | "details" | "p" | "bdi" | "time" | "area" | "br" | "var"
                    | "aside" | "main" | "tfoot" | "hr" | "label" | "rp" | "menuitem" => {
                        x += 1;
                    }
                    "portal" | "wbr" | "cite" | "ins" | "footer" | "table" | "address" | "div"
                    | "optgroup" | "dd" | "samp" | "map" | "xmp" | "embed" | "strong"
                    | "dialog" | "colgroup" | "input" | "figure" | "body" | "strike" | "audio"
                    | "marquee" | "noscript" | "form" => {
                        x += 2;
                    }
                    "nobr" | "font" | "textarea" | "tbody" | "picture" | "legend" | "img"
                    | "progress" | "meter" | "script" | "dt" | "summary" | "ol" | "acronym"
                    | "header" | "title" | "span" | "abbr" | "hgroup" | "meta" | "plaintext"
                    | "base" | "sub" | "select" | "s" => {
                        x += 3;
                    }
                    "output" | "datalist" | "article" | "param" | "blockquote" | "i" | "tr"
                    | "html" | "section" | "link" | "small" | "canvas" | "option" | "dir"
                    | "col" | "noembed" | "rtc" | "big" | "figcaption" | "kbd" | "b" | "u"
                    | "a" | "td" | "center" => {
                        x += 4;
                    }
                    "menu" | "template" | "data" | "image" | "fieldset" | "slot" | "q"
                    | "thead" | "nav" | "style" | "button" | "video" | "dl" | "caption"
                    | "ruby" | "tt" | "dfn" | "code" | "source" | "h1" | "iframe" | "sup"
                    | "noframes" | "frameset" | "track" | "frame" | "rt" | "object" => {
                        x += 5;
                    }
                    _ => {}
                }
            }
            x
        });
    });

    group.bench_function("match_0", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &html_elements {
                match s.as_str() {
                    "bdo" | "rb" | "th" | "ul" | "pre" | "mark" | "em" | "search" | "head"
                    | "li" | "del" | "details" | "p" | "bdi" | "time" | "area" | "br" | "var"
                    | "aside" | "main" | "tfoot" | "hr" | "label" | "rp" | "menuitem" => {
                        x += 0;
                    }
                    "portal" | "wbr" | "cite" | "ins" | "footer" | "table" | "address" | "div"
                    | "optgroup" | "dd" | "samp" | "map" | "xmp" | "embed" | "strong"
                    | "dialog" | "colgroup" | "input" | "figure" | "body" | "strike" | "audio"
                    | "marquee" | "noscript" | "form" => {
                        x += 1;
                    }
                    "nobr" | "font" | "textarea" | "tbody" | "picture" | "legend" | "img"
                    | "progress" | "meter" | "script" | "dt" | "summary" | "ol" | "acronym"
                    | "header" | "title" | "span" | "abbr" | "hgroup" | "meta" | "plaintext"
                    | "base" | "sub" | "select" | "s" => {
                        x += 2;
                    }
                    "output" | "datalist" | "article" | "param" | "blockquote" | "i" | "tr"
                    | "html" | "section" | "link" | "small" | "canvas" | "option" | "dir"
                    | "col" | "noembed" | "rtc" | "big" | "figcaption" | "kbd" | "b" | "u"
                    | "a" | "td" | "center" => {
                        x += 3;
                    }
                    "menu" | "template" | "data" | "image" | "fieldset" | "slot" | "q"
                    | "thead" | "nav" | "style" | "button" | "video" | "dl" | "caption"
                    | "ruby" | "tt" | "dfn" | "code" | "source" | "h1" | "iframe" | "sup"
                    | "noframes" | "frameset" | "track" | "frame" | "rt" | "object" => {
                        x += 4;
                    }
                    _ => {}
                }
            }
            x
        });
    });

    group.bench_function("trie_match_rand", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &html_elements {
                trie_match!(match s.as_str() {
                    "bdo" | "rb" | "th" | "ul" | "pre" | "mark" | "em" | "search" | "head"
                    | "li" | "del" | "details" | "p" | "bdi" | "time" | "area" | "br" | "var"
                    | "aside" | "main" | "tfoot" | "hr" | "label" | "rp" | "menuitem" => {
                        x += 3141;
                    }
                    "portal" | "wbr" | "cite" | "ins" | "footer" | "table" | "address" | "div"
                    | "optgroup" | "dd" | "samp" | "map" | "xmp" | "embed" | "strong"
                    | "dialog" | "colgroup" | "input" | "figure" | "body" | "strike" | "audio"
                    | "marquee" | "noscript" | "form" => {
                        x += 5926;
                    }
                    "nobr" | "font" | "textarea" | "tbody" | "picture" | "legend" | "img"
                    | "progress" | "meter" | "script" | "dt" | "summary" | "ol" | "acronym"
                    | "header" | "title" | "span" | "abbr" | "hgroup" | "meta" | "plaintext"
                    | "base" | "sub" | "select" | "s" => {
                        x += 5358;
                    }
                    "output" | "datalist" | "article" | "param" | "blockquote" | "i" | "tr"
                    | "html" | "section" | "link" | "small" | "canvas" | "option" | "dir"
                    | "col" | "noembed" | "rtc" | "big" | "figcaption" | "kbd" | "b" | "u"
                    | "a" | "td" | "center" => {
                        x += 9793;
                    }
                    "menu" | "template" | "data" | "image" | "fieldset" | "slot" | "q"
                    | "thead" | "nav" | "style" | "button" | "video" | "dl" | "caption"
                    | "ruby" | "tt" | "dfn" | "code" | "source" | "h1" | "iframe" | "sup"
                    | "noframes" | "frameset" | "track" | "frame" | "rt" | "object" => {
                        x += 2384;
                    }
                    _ => {}
                })
            }
            x
        });
    });

    group.bench_function("trie_match_1", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &html_elements {
                trie_match!(match s.as_str() {
                    "bdo" | "rb" | "th" | "ul" | "pre" | "mark" | "em" | "search" | "head"
                    | "li" | "del" | "details" | "p" | "bdi" | "time" | "area" | "br" | "var"
                    | "aside" | "main" | "tfoot" | "hr" | "label" | "rp" | "menuitem" => {
                        x += 1;
                    }
                    "portal" | "wbr" | "cite" | "ins" | "footer" | "table" | "address" | "div"
                    | "optgroup" | "dd" | "samp" | "map" | "xmp" | "embed" | "strong"
                    | "dialog" | "colgroup" | "input" | "figure" | "body" | "strike" | "audio"
                    | "marquee" | "noscript" | "form" => {
                        x += 2;
                    }
                    "nobr" | "font" | "textarea" | "tbody" | "picture" | "legend" | "img"
                    | "progress" | "meter" | "script" | "dt" | "summary" | "ol" | "acronym"
                    | "header" | "title" | "span" | "abbr" | "hgroup" | "meta" | "plaintext"
                    | "base" | "sub" | "select" | "s" => {
                        x += 3;
                    }
                    "output" | "datalist" | "article" | "param" | "blockquote" | "i" | "tr"
                    | "html" | "section" | "link" | "small" | "canvas" | "option" | "dir"
                    | "col" | "noembed" | "rtc" | "big" | "figcaption" | "kbd" | "b" | "u"
                    | "a" | "td" | "center" => {
                        x += 4;
                    }
                    "menu" | "template" | "data" | "image" | "fieldset" | "slot" | "q"
                    | "thead" | "nav" | "style" | "button" | "video" | "dl" | "caption"
                    | "ruby" | "tt" | "dfn" | "code" | "source" | "h1" | "iframe" | "sup"
                    | "noframes" | "frameset" | "track" | "frame" | "rt" | "object" => {
                        x += 5;
                    }
                    _ => {}
                })
            }
            x
        });
    });

    group.bench_function("trie_match_0", |b| {
        b.iter(|| {
            let mut x = 0;
            for s in &html_elements {
                trie_match!(match s.as_str() {
                    "bdo" | "rb" | "th" | "ul" | "pre" | "mark" | "em" | "search" | "head"
                    | "li" | "del" | "details" | "p" | "bdi" | "time" | "area" | "br" | "var"
                    | "aside" | "main" | "tfoot" | "hr" | "label" | "rp" | "menuitem" => {
                        x += 0;
                    }
                    "portal" | "wbr" | "cite" | "ins" | "footer" | "table" | "address" | "div"
                    | "optgroup" | "dd" | "samp" | "map" | "xmp" | "embed" | "strong"
                    | "dialog" | "colgroup" | "input" | "figure" | "body" | "strike" | "audio"
                    | "marquee" | "noscript" | "form" => {
                        x += 1;
                    }
                    "nobr" | "font" | "textarea" | "tbody" | "picture" | "legend" | "img"
                    | "progress" | "meter" | "script" | "dt" | "summary" | "ol" | "acronym"
                    | "header" | "title" | "span" | "abbr" | "hgroup" | "meta" | "plaintext"
                    | "base" | "sub" | "select" | "s" => {
                        x += 2;
                    }
                    "output" | "datalist" | "article" | "param" | "blockquote" | "i" | "tr"
                    | "html" | "section" | "link" | "small" | "canvas" | "option" | "dir"
                    | "col" | "noembed" | "rtc" | "big" | "figcaption" | "kbd" | "b" | "u"
                    | "a" | "td" | "center" => {
                        x += 3;
                    }
                    "menu" | "template" | "data" | "image" | "fieldset" | "slot" | "q"
                    | "thead" | "nav" | "style" | "button" | "video" | "dl" | "caption"
                    | "ruby" | "tt" | "dfn" | "code" | "source" | "h1" | "iframe" | "sup"
                    | "noframes" | "frameset" | "track" | "frame" | "rt" | "object" => {
                        x += 4;
                    }
                    _ => {}
                })
            }
            x
        });
    });
}

criterion_group!(benches, criterion_word100, criterion_html_elements);

criterion_main!(benches);
