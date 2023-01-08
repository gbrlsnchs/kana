use phf::{phf_map as map, phf_set as set, Map, Set};

pub static SYLLABARY: Map<&'static str, &'static str> = map! {
	// Vowels
	"a" => "ア",
	"i" => "イ",
	"u" => "ウ",
	"e" => "エ",
	"o" => "オ",

	// K
	"ka" => "カ",
	"ki" => "キ",
	"ku" => "ク",
	"ke" => "ケ",
	"ko" => "コ",
	"kya" => "キャ",
	"kyu" => "キュ",
	"kyo" => "キョ",

	// S
	"sa" => "サ",
	"shi" => "シ",
	"su" => "ス",
	"se" => "セ",
	"so" => "ソ",
	"sha" => "シャ",
	"shu" => "シュ",
	"sho" => "ショ",

	// T
	"ta" => "タ",
	"chi" => "チ",
	"tsu" => "ツ",
	"te" => "テ",
	"to" => "ト",
	"cha" => "チャ",
	"chu" => "チュ",
	"cho" => "チョ",

	// N
	"na" => "ナ",
	"ni" => "ニ",
	"nu" => "ヌ",
	"ne" => "ネ",
	"no" => "ノ",
	"nya" => "ニャ",
	"nyu" => "ニュ",
	"nyo" => "ニョ",

	// H
	"ha" => "ハ",
	"hi" => "ヒ",
	"fu" => "フ",
	"he" => "ヘ",
	"ho" => "ホ",
	"hya" => "ヒャ",
	"hyu" => "ヒュ",
	"hyo" => "ヒョ",

	// M
	"ma" => "マ",
	"mi" => "ミ",
	"mu" => "ム",
	"me" => "メ",
	"mo" => "モ",
	"mya" => "ミャ",
	"myu" => "ミュ",
	"myo" => "ミョ",

	// Y
	"ya" => "ヤ",
	"yu" => "ユ",
	"yo" => "ヨ",

	// R
	"ra" => "ラ",
	"ri" => "リ",
	"ru" => "ル",
	"re" => "レ",
	"ro" => "ロ",
	"rya" => "リャ",
	"ryu" => "リュ",
	"ryo" => "リョ",

	// W
	"wa" => "ワ",
	"wi" => "ヰ",
	"we" => "ヱ",
	"wo" => "ヲ",

	// G
	"ga" => "ガ",
	"gi" => "ギ",
	"gu" => "グ",
	"ge" => "ゲ",
	"go" => "ゴ",
	"gya" => "ギャ",
	"gyu" => "ギュ",
	"gyo" => "ギョ",

	// Z
	"za" => "ザ",
	"ji" => "ジ",
	"zu" => "ズ",
	"ze" => "ゼ",
	"zo" => "ゾ",
	"ja" => "ジャ",
	"ju" => "ジュ",
	"jo" => "ジョ",

	// D
	"da" => "ダ",
	"dji" => "ヂ",
	"dzu" => "ヅ",
	"de" => "デ",
	"do" => "ド",
	"dja" => "ヂャ",
	"dju" => "ヂュ",
	"djo" => "ヂョ",

	// B
	"ba" => "バ",
	"bi" => "ビ",
	"bu" => "ブ",
	"be" => "ベ",
	"bo" => "ボ",
	"bya" => "ビャ",
	"byu" => "ビュ",
	"byo" => "ビョ",

	// P
	"pa" => "パ",
	"pi" => "ピ",
	"pu" => "プ",
	"pe" => "ペ",
	"po" => "ポ",
	"pya" => "ピャ",
	"pyu" => "ピュ",
	"pyo" => "ピョ",

	"n" => "ン",
};

pub static EXTENDED_SYLLABARY: Map<&'static str, &'static str> = map! {
	"yi" => "イィ",
	"ye" => "イェ",
	"wi" => "ウィ",
	"we" => "ウェ",
	"wo" => "ウォ",
	"wu" => "ウゥ",
	"va" => "ヴァ",
	"vi" => "ヴィ",
	"vu" => "ヴ",
	"ve" => "ヴェ",
	"vo" => "ヴォ",
	"vya" => "ヴャ",
	"vyu" => "ヴュ",
	"vye" => "ヴィェ",
	"vyo" => "ヴョ",
	"kye" => "キェ",
	"gye" => "ギェ",
	"kwa" => "クァ",
	"kwi" => "クィ",
	"kwe" => "クェ",
	"kwo" => "クォ",
	"gwa" => "グァ",
	"gwi" => "グィ",
	"gwe" => "グェ",
	"gwo" => "グォ",
	"she" => "シェ",
	"je" => "ジェ",
	"si" => "スィ",
	"zi" => "ズィ",
	"tsa" => "ツァ",
	"tsi" => "ツィ",
	"tse" => "ツェ",
	"tso" => "ツォ",
	"tsyu" => "ツュ",
	"ti" => "ティ",
	"tu" => "トゥ",
	"tyu" => "テュ",
	"di" => "ディ",
	"du" => "ドゥ",
	"dyu" => "デュ",
	"nye" => "ニェ",
	"hye" => "ヒェ",
	"bye" => "ビェ",
	"pye" => "ピェ",
	"fa" => "ファ",
	"fi" => "フィ",
	"fe" => "フェ",
	"fo" => "フォ",
	"fya" => "フャ",
	"fyu" => "フュ",
	"fye" => "フィェ",
	"fyo" => "フョ",
	"hu" => "ホゥ",
	"mye" => "ミェ",
	"rye" => "リェ",
	"la" => "ラ゜",
	"li" => "リ゜",
	"lu" => "ル゜",
	"le" => "レ゜",
	"lo" => "ロ゜",
	"lya" => "リ゜ャ",
	"lyu" => "リ゜ュ",
	"lye" => "リ゜ェ",
	"lyo" => "リ゜ョ",
};

pub static SMALL_VOWELS: Map<&'static str, &'static str> = map! {
	"a" => "ァ",
	"i" => "ィ",
	"u" => "ゥ",
	"e" => "ェ",
	"o" => "ォ",
};

pub const SOKUON_GRAPH: &'static str = "ッ";
pub static SOKUON_MATCHES: Set<&'static str> = set! {
	"kk",
	"cc",
	"dd",
	"gg",
	"pp",
	"ss",
	"tt",
};

pub const CHOUONPU_GRAPH: &'static str = "ー";
pub static CHOUONPU_MATCHES: Set<&'static str> = set! {
	"aa",
	"ii",
	"uu",
	"ee",
	"oo",
};
