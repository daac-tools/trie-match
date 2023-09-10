use std::time::Duration;

use trie_match::trie_match;

use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};

static WORDS_100: &[&str] = &[
    "stampeding",
    "commendable",
    "adrenaline",
    "exobiology",
    "indifference",
    "avuncular",
    "prevailed",
    "foreparts",
    "legalistically",
    "intermarries",
    "desideratum",
    "evaluating",
    "lavishing",
    "attractable",
    "philippics",
    "antiabortionist",
    "lascivious",
    "breathable",
    "histogram",
    "rattlings",
    "interdict",
    "summarized",
    "relieving",
    "congresspeople",
    "fitfulness",
    "percolation",
    "upperclasswoman",
    "epistemic",
    "Chantilly",
    "stonemasons",
    "nonferrous",
    "emulsions",
    "charitably",
    "barracudas",
    "integrity",
    "knockdowns",
    "roadworks",
    "abortionists",
    "Salvadoran",
    "chanceries",
    "misinform",
    "caretaker",
    "extricated",
    "mandolins",
    "steeliest",
    "transpiration",
    "weirdness",
    "audiologists",
    "baronetcies",
    "performing",
    "publishing",
    "suspending",
    "dermatological",
    "contemplate",
    "spiritless",
    "nightwatchman",
    "paradisaical",
    "implicating",
    "timpanists",
    "Leavenworth",
    "amorality",
    "strangulated",
    "cellophane",
    "waterboard",
    "astrophysicists",
    "aerospace",
    "passphrase",
    "engendered",
    "spotlighting",
    "misapplication",
    "barterers",
    "poetesses",
    "dollhouse",
    "laparoscopic",
    "Dubrovnik",
    "rerecords",
    "shielding",
    "orthographically",
    "thicknesses",
    "Bendictus",
    "congealed",
    "cooperative",
    "encompass",
    "grouching",
    "shipowners",
    "jealously",
    "generational",
    "antecedents",
    "persecutes",
    "exemplified",
    "admirable",
    "squeakiest",
    "absconding",
    "extirpated",
    "exoskeletons",
    "earthworms",
    "chaotically",
    "shipbuilder",
    "equidistantly",
    "overprint",
];

static HTML_ELEMENTS: &[&str] = &[
    "tt",
    "optgroup",
    "p",
    "map",
    "figcaption",
    "portal",
    "table",
    "strike",
    "colgroup",
    "style",
    "meter",
    "option",
    "dd",
    "img",
    "bdo",
    "samp",
    "track",
    "u",
    "span",
    "plaintext",
    "article",
    "center",
    "frameset",
    "cite",
    "datalist",
    "big",
    "select",
    "caption",
    "dialog",
    "section",
    "q",
    "base",
    "summary",
    "object",
    "td",
    "input",
    "tbody",
    "wbr",
    "dfn",
    "image",
    "figure",
    "output",
    "head",
    "iframe",
    "tfoot",
    "acronym",
    "ins",
    "br",
    "del",
    "rt",
    "noembed",
    "menu",
    "search",
    "main",
    "small",
    "param",
    "tr",
    "template",
    "ol",
    "kbd",
    "strong",
    "h1",
    "slot",
    "ul",
    "button",
    "video",
    "xmp",
    "th",
    "aside",
    "font",
    "rp",
    "data",
    "dt",
    "abbr",
    "pre",
    "audio",
    "fieldset",
    "source",
    "link",
    "nav",
    "meta",
    "blockquote",
    "picture",
    "form",
    "bdi",
    "a",
    "textarea",
    "s",
    "canvas",
    "mark",
    "menuitem",
    "li",
    "noscript",
    "code",
    "area",
    "rb",
    "b",
    "details",
    "label",
    "progress",
    "sup",
    "title",
    "html",
    "em",
    "i",
    "embed",
    "nobr",
    "div",
    "time",
    "rtc",
    "frame",
    "dl",
    "address",
    "noframes",
    "footer",
    "sub",
    "var",
    "col",
    "dir",
    "header",
    "legend",
    "hgroup",
    "marquee",
    "hr",
    "thead",
    "body",
    "script",
    "ruby",
];

fn criterion_word100(c: &mut Criterion) {
    let mut group = c.benchmark_group("words_100");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(5));
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("match_rand", |b| {
        b.iter(|| {
            let mut x = 0;
            for &s in WORDS_100 {
                match std::hint::black_box(s) {
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
            for &s in WORDS_100 {
                match std::hint::black_box(s) {
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
            for &s in WORDS_100 {
                match std::hint::black_box(s) {
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
            for &s in WORDS_100 {
                trie_match!(match std::hint::black_box(s) {
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
            for &s in WORDS_100 {
                trie_match!(match std::hint::black_box(s) {
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
            for &s in WORDS_100 {
                trie_match!(match std::hint::black_box(s) {
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

    group.bench_function("match_rand", |b| {
        b.iter(|| {
            let mut x = 0;
            for &s in HTML_ELEMENTS {
                match std::hint::black_box(s) {
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
            for &s in HTML_ELEMENTS {
                match std::hint::black_box(s) {
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
            for &s in HTML_ELEMENTS {
                match std::hint::black_box(s) {
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
            for &s in HTML_ELEMENTS {
                trie_match!(match std::hint::black_box(s) {
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
            for &s in HTML_ELEMENTS {
                trie_match!(match std::hint::black_box(s) {
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
            for &s in HTML_ELEMENTS {
                trie_match!(match std::hint::black_box(s) {
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
