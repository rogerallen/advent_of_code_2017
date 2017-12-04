use std::env;
use std::process;

// ======================================================================
// can you tell this is my first rust code?  :-)
// ======================================================================
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Advent of Code 2017");
    if args.len() != 2 {
        usage();
    }
    //println!("Args: {:?}", &args);
    let day = &args[1];
    match day as &str {
        "1" => { day01(); }
        "2" => { day02(); }
        _ => { usage(); }
    }
}

fn usage() {
    let args: Vec<String> = env::args().collect();
    println!("Usage: {} <day>",args[0]);
    process::exit(1);
}

// ======================================================================
// Day 1
// ======================================================================

fn day01() {
    println!("Day 1\n");

    println!("Part One");

    println!("Testing...");
    let test_strings = vec![ "1122", "1111", "1234", "91212129"];
    let test_answers = vec![ 3, 4, 0, 9];
    for (i,s) in test_strings.iter().enumerate() {
        let a = captcha(&s);
        let a_correct = test_answers[i];
        print!("  captcha({:?}) == {:?}",&s,&a);
        if a != a_correct {
            println!(" FAIL");
        } else {
            println!("");
        }
    }

    println!("Solution...");
    let test_input = "9513446799636685297929646689682997114316733445451534532351778534251427172168183621874641711534917291674333857423799375512628489423332297538215855176592633692631974822259161766238385922277893623911332569448978771948316155868781496698895492971356383996932885518732997624253678694279666572149831616312497994856288871586777793459926952491318336997159553714584541897294117487641872629796825583725975692264125865827534677223541484795877371955124463989228886498682421539667224963783616245646832154384756663251487668681425754536722827563651327524674183443696227523828832466473538347472991998913211857749878157579176457395375632995576569388455888156465451723693767887681392547189273391948632726499868313747261828186732986628365773728583387184112323696592536446536231376615949825166773536471531487969852535699774113163667286537193767515119362865141925612849443983484245268194842563154567638354645735331855896155142741664246715666899824364722914296492444672653852387389477634257768229772399416521198625393426443499223611843766134883441223328256883497423324753229392393974622181429913535973327323952241674979677481518733692544535323219895684629719868384266425386835539719237716339198485163916562434854579365958111931354576991558771236977242668756782139961638347251644828724786827751748399123668854393894787851872256667336215726674348886747128237416273154988619267824361227888751562445622387695218161341884756795223464751862965655559143779425283154533252573949165492138175581615176611845489857169132936848668646319955661492488428427435269169173654812114842568381636982389224236455633316898178163297452453296667661849622174541778669494388167451186352488555379581934999276412919598411422973399319799937518713422398874326665375216437246445791623283898584648278989674418242112957668397484671119761553847275799873495363759266296477844157237423239163559391553961176475377151369399646747881452252547741718734949967752564774161341784833521492494243662658471121369649641815562327698395293573991648351369767162642763475561544795982183714447737149239846151871434656618825566387329765118727515699213962477996399781652131918996434125559698427945714572488376342126989157872118279163127742349";
    println!("  answer is {:?}",captcha(&test_input));

    println!("\nPart Two");

    println!("Testing...");
    let test_strings = vec![ "1212", "1221", "123425", "123123", "12131415"];
    let test_answers = vec![     6,      0,        4,       12,          4];
    for (i,s) in test_strings.iter().enumerate() {
        let a = captcha2(&s);
        let a_correct = test_answers[i];
        print!("  captcha2({:?}) == {:?}",&s,&a);
        if a != a_correct {
            println!(" FAIL");
        } else {
            println!("");
        }
    }

    println!("Solution...");
    println!("  answer is {:?}",captcha2(&test_input));

}

fn captcha(s: &str) -> i32 {
    let s_digits = str_to_digits(&s);
    //println!("{:?}",&s_digits);
    let mut sum_digits = Vec::new();
    for (i,x) in s_digits.iter().enumerate() {
        let j = (i+1)%s_digits.len();
        if x == &s_digits[j] {
            sum_digits.push(*x);
        }
    }
    let sum = sum_digits.iter().sum();
    return sum;
}

fn captcha2(s: &str) -> i32 {
    let s_digits = str_to_digits(&s);
    //println!("{:?}",&s_digits);
    let mut sum_digits = Vec::new();
    for (i,x) in s_digits.iter().enumerate() {
        let h = s_digits.len() / 2;
        let j = (i+h)%s_digits.len();
        if x == &s_digits[j] {
            sum_digits.push(*x);
        }
    }
    let sum = sum_digits.iter().sum();
    return sum;
}

fn str_to_digits(s: &str) -> Vec<i32> {
    return s.chars()
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect();
}

// ======================================================================
// Day 2
// ======================================================================

fn day02() {
    println!("Day 2\n");

    println!("Part One");

    println!("Testing...");
    let mut test_input = Vec::new();
    test_input.push(vec![5, 1,  9, 5]);
    test_input.push(vec![7, 5, 3]);
    test_input.push(vec![2, 4,  6, 8]);
    println!("  {:?}",&test_input);
    let test_answer = checksum(&test_input);
    if test_answer != 18 {
        println!("  test FAIL");
    } else {
        println!("  test pass")
    }

    println!("Solution...");

    let mut real_test_input = Vec::new();
    real_test_input.push(vec![116,	1259,	1045,	679,	1334,	157,	277,	1217,	218,	641,	1089,	136,	247,	1195,	239,	834]);
    real_test_input.push(vec![269,	1751,	732,	3016,	260,	6440,	5773,	4677,	306,	230,	6928,	7182,	231,	2942,	2738,	3617]);
    real_test_input.push(vec![644,	128,	89,	361,	530,	97,	35,	604,	535,	297,	599,	121,	567,	106,	114,	480]);
    real_test_input.push(vec![105,	408,	120,	363,	430,	102,	137,	283,	123,	258,	19,	101,	181,	477,	463,	279]);
    real_test_input.push(vec![873,	116,	840,	105,	285,	238,	540,	22,	117,	125,	699,	953,	920,	106,	113,	259]);
    real_test_input.push(vec![3695,	161,	186,	2188,	3611,	2802,	157,	2154,	3394,	145,	2725,	1327,	3741,	2493,	3607,	4041]);
    real_test_input.push(vec![140,	1401,	110,	119,	112,	1586,	125,	937,	1469,	1015,	879,	1798,	122,	1151,	100,	926]);
    real_test_input.push(vec![2401,	191,	219,	607,	267,	2362,	932,	2283,	889,	2567,	2171,	2409,	1078,	2247,	2441,	245]);
    real_test_input.push(vec![928,	1142,	957,	1155,	922,	1039,	452,	285,	467,	305,	506,	221,	281,	59,	667,	232]);
    real_test_input.push(vec![3882,	1698,	170,	5796,	2557,	173,	1228,	4630,	174,	3508,	5629,	4395,	180,	5100,	2814,	2247]);
    real_test_input.push(vec![396,	311,	223,	227,	340,	313,	355,	469,	229,	162,	107,	76,	363,	132,	453,	161]);
    real_test_input.push(vec![627,	1331,	1143,	1572,	966,	388,	198,	2068,	201,	239,	176,	1805,	1506,	1890,	1980,	1887]);
    real_test_input.push(vec![3390,	5336,	1730,	4072,	5342,	216,	3823,	85,	5408,	5774,	247,	5308,	232,	256,	5214,	787]);
    real_test_input.push(vec![176,	1694,	1787,	1586,	3798,	4243,	157,	4224,	3603,	2121,	3733,	851,	2493,	4136,	148,	153]);
    real_test_input.push(vec![2432,	4030,	3397,	4032,	3952,	2727,	157,	3284,	3450,	3229,	4169,	3471,	4255,	155,	127,	186]);
    real_test_input.push(vec![919,	615,	335,	816,	138,	97,	881,	790,	855,	89,	451,	789,	423,	108,	95,	116]);
    let test_answer = checksum(&real_test_input);
    println!("  answer = {:?}",&test_answer);

    println!("\nPart Two");

    println!("Testing...");
    let mut test_input = Vec::new();
    test_input.push(vec![5, 9, 2, 8]);
    test_input.push(vec![9, 4, 7, 3]);
    test_input.push(vec![3, 8, 6, 5]);
    println!("  {:?}",&test_input);
    let test_answer = checksum2(&test_input);
    if test_answer != 9 {
        println!("  test FAIL");
    } else {
        println!("  test pass")
    }

    println!("Solution...");
    let test_answer = checksum2(&real_test_input);
    println!("  answer = {:?}",&test_answer);

}

fn checksum(rows: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in rows {
        sum += mmdiff(&row);
    }
    return sum;
}

fn mmdiff(row: &Vec<i32>) -> i32 {
    let mx;
    let mn;
    match row.iter().max() {
        None => { println!{"unhandled error!"}; return -1; }
        Some(i) => { mx = *i; }
    }
    match row.iter().min() {
        None => { println!{"unhandled error!"}; return -1; }
        Some(i) => { mn = *i; }
    }
    return mx - mn;
}

fn checksum2(rows: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in rows {
        sum += div2val(&row);
    }
    return sum;
}

// "find the only two numbers in each row where one evenly divides the other"
fn div2val(row: &Vec<i32>) -> i32 {
    for (i,x) in row.iter().enumerate() {
        for (j,y) in row.iter().enumerate() {
            if i != j {
                if x % y == 0 {
                    // going to assume first match is all we need
                    return x/y;
                }
            }
        }
    }
    return -1;
}
