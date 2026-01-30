use std::collections::HashMap;

pub fn create_discord_emoji_map() -> HashMap<String, String> {
    let mut emojis = HashMap::new();
    
    // Smileys & Emotion
    emojis.insert("grinning".to_string(), "😀".to_string());
    emojis.insert("smiley".to_string(), "😃".to_string());
    emojis.insert("smile".to_string(), "😄".to_string());
    emojis.insert("grin".to_string(), "😁".to_string());
    emojis.insert("laughing".to_string(), "😆".to_string());
    emojis.insert("sweat_smile".to_string(), "😅".to_string());
    emojis.insert("joy".to_string(), "😂".to_string());
    emojis.insert("rofl".to_string(), "🤣".to_string());
    emojis.insert("smiling_face_with_tear".to_string(), "🥲".to_string());
    emojis.insert("relaxed".to_string(), "☺️".to_string());
    emojis.insert("blush".to_string(), "😊".to_string());
    emojis.insert("innocent".to_string(), "😇".to_string());
    emojis.insert("slightly_smiling_face".to_string(), "🙂".to_string());
    emojis.insert("upside_down_face".to_string(), "🙃".to_string());
    emojis.insert("wink".to_string(), "😉".to_string());
    emojis.insert("relieved".to_string(), "😌".to_string());
    emojis.insert("heart_eyes".to_string(), "😍".to_string());
    emojis.insert("smiling_face_with_3_hearts".to_string(), "🥰".to_string());
    emojis.insert("kissing_heart".to_string(), "😘".to_string());
    emojis.insert("kissing".to_string(), "😗".to_string());
    emojis.insert("kissing_smiling_eyes".to_string(), "😙".to_string());
    emojis.insert("kissing_closed_eyes".to_string(), "😚".to_string());
    emojis.insert("yum".to_string(), "😋".to_string());
    emojis.insert("stuck_out_tongue".to_string(), "😛".to_string());
    emojis.insert("stuck_out_tongue_winking_eye".to_string(), "😜".to_string());
    emojis.insert("stuck_out_tongue_closed_eyes".to_string(), "😝".to_string());
    emojis.insert("money_mouth_face".to_string(), "🤑".to_string());
    emojis.insert("hugging_face".to_string(), "🤗".to_string());
    emojis.insert("face_with_hand_over_mouth".to_string(), "🤭".to_string());
    emojis.insert("shushing_face".to_string(), "🤫".to_string());
    emojis.insert("thinking_face".to_string(), "🤔".to_string());
    emojis.insert("zipper_mouth_face".to_string(), "🤐".to_string());
    emojis.insert("face_with_raised_eyebrow".to_string(), "🤨".to_string());
    emojis.insert("neutral_face".to_string(), "😐".to_string());
    emojis.insert("expressionless".to_string(), "😑".to_string());
    emojis.insert("no_mouth".to_string(), "😶".to_string());
    emojis.insert("smirk".to_string(), "😏".to_string());
    emojis.insert("unamused".to_string(), "😒".to_string());
    emojis.insert("roll_eyes".to_string(), "🙄".to_string());
    emojis.insert("grimacing".to_string(), "😬".to_string());
    emojis.insert("lying_face".to_string(), "🤥".to_string());
    emojis.insert("pensive".to_string(), "😔".to_string());
    emojis.insert("sleepy".to_string(), "😪".to_string());
    emojis.insert("drooling_face".to_string(), "🤤".to_string());
    emojis.insert("sleeping".to_string(), "😴".to_string());
    emojis.insert("mask".to_string(), "😷".to_string());
    emojis.insert("face_with_thermometer".to_string(), "🤒".to_string());
    emojis.insert("face_with_head_bandage".to_string(), "🤕".to_string());
    emojis.insert("nauseated_face".to_string(), "🤢".to_string());
    emojis.insert("face_vomiting".to_string(), "🤮".to_string());
    emojis.insert("sneezing_face".to_string(), "🤧".to_string());
    emojis.insert("hot_face".to_string(), "🥵".to_string());
    emojis.insert("cold_face".to_string(), "🥶".to_string());
    emojis.insert("woozy_face".to_string(), "🥴".to_string());
    emojis.insert("dizzy_face".to_string(), "😵".to_string());
    emojis.insert("exploding_head".to_string(), "🤯".to_string());
    emojis.insert("cowboy_hat_face".to_string(), "🤠".to_string());
    emojis.insert("partying_face".to_string(), "🥳".to_string());
    emojis.insert("sunglasses".to_string(), "😎".to_string());
    emojis.insert("nerd_face".to_string(), "🤓".to_string());
    emojis.insert("face_with_monocle".to_string(), "🧐".to_string());
    emojis.insert("confused".to_string(), "😕".to_string());
    emojis.insert("worried".to_string(), "😟".to_string());
    emojis.insert("slightly_frowning_face".to_string(), "🙁".to_string());
    emojis.insert("frowning_face".to_string(), "☹️".to_string());
    emojis.insert("open_mouth".to_string(), "😮".to_string());
    emojis.insert("hushed".to_string(), "😯".to_string());
    emojis.insert("astonished".to_string(), "😲".to_string());
    emojis.insert("flushed".to_string(), "😳".to_string());
    emojis.insert("pleading_face".to_string(), "🥺".to_string());
    emojis.insert("frowning".to_string(), "😦".to_string());
    emojis.insert("anguished".to_string(), "😧".to_string());
    emojis.insert("fearful".to_string(), "😨".to_string());
    emojis.insert("cold_sweat".to_string(), "😰".to_string());
    emojis.insert("disappointed_relieved".to_string(), "😥".to_string());
    emojis.insert("cry".to_string(), "😢".to_string());
    emojis.insert("sob".to_string(), "😭".to_string());
    emojis.insert("scream".to_string(), "😱".to_string());
    emojis.insert("confounded".to_string(), "😖".to_string());
    emojis.insert("persevere".to_string(), "😣".to_string());
    emojis.insert("disappointed".to_string(), "😞".to_string());
    emojis.insert("sweat".to_string(), "😓".to_string());
    emojis.insert("weary".to_string(), "😩".to_string());
    emojis.insert("tired_face".to_string(), "😫".to_string());
    emojis.insert("yawning_face".to_string(), "🥱".to_string());
    emojis.insert("triumph".to_string(), "😤".to_string());
    emojis.insert("rage".to_string(), "😡".to_string());
    emojis.insert("angry".to_string(), "😠".to_string());
    emojis.insert("face_with_symbols_on_mouth".to_string(), "🤬".to_string());
    emojis.insert("smiling_imp".to_string(), "😈".to_string());
    emojis.insert("imp".to_string(), "👿".to_string());
    emojis.insert("skull".to_string(), "💀".to_string());
    emojis.insert("skull_and_crossbones".to_string(), "☠️".to_string());
    
    // Gestures & People
    emojis.insert("wave".to_string(), "👋".to_string());
    emojis.insert("raised_back_of_hand".to_string(), "🤚".to_string());
    emojis.insert("raised_hand_with_fingers_splayed".to_string(), "🖐️".to_string());
    emojis.insert("hand".to_string(), "✋".to_string());
    emojis.insert("vulcan_salute".to_string(), "🖖".to_string());
    emojis.insert("ok_hand".to_string(), "👌".to_string());
    emojis.insert("pinching_hand".to_string(), "🤏".to_string());
    emojis.insert("v".to_string(), "✌️".to_string());
    emojis.insert("crossed_fingers".to_string(), "🤞".to_string());
    emojis.insert("metal".to_string(), "🤘".to_string());
    emojis.insert("call_me_hand".to_string(), "🤙".to_string());
    emojis.insert("point_left".to_string(), "👈".to_string());
    emojis.insert("point_right".to_string(), "👉".to_string());
    emojis.insert("point_up_2".to_string(), "👆".to_string());
    emojis.insert("point_down".to_string(), "👇".to_string());
    emojis.insert("point_up".to_string(), "☝️".to_string());
    emojis.insert("+1".to_string(), "👍".to_string());
    emojis.insert("-1".to_string(), "👎".to_string());
    emojis.insert("fist".to_string(), "✊".to_string());
    emojis.insert("facepunch".to_string(), "👊".to_string());
    emojis.insert("clap".to_string(), "👏".to_string());
    emojis.insert("raised_hands".to_string(), "🙌".to_string());
    emojis.insert("open_hands".to_string(), "👐".to_string());
    emojis.insert("palms_up_together".to_string(), "🤲".to_string());
    emojis.insert("handshake".to_string(), "🤝".to_string());
    emojis.insert("pray".to_string(), "🙏".to_string());
    
    // Hearts
    emojis.insert("heart".to_string(), "❤️".to_string());
    emojis.insert("orange_heart".to_string(), "🧡".to_string());
    emojis.insert("yellow_heart".to_string(), "💛".to_string());
    emojis.insert("green_heart".to_string(), "💚".to_string());
    emojis.insert("blue_heart".to_string(), "💙".to_string());
    emojis.insert("purple_heart".to_string(), "💜".to_string());
    emojis.insert("black_heart".to_string(), "🖤".to_string());
    emojis.insert("brown_heart".to_string(), "🤎".to_string());
    emojis.insert("white_heart".to_string(), "🤍".to_string());
    emojis.insert("broken_heart".to_string(), "💔".to_string());
    emojis.insert("heart_exclamation".to_string(), "❣️".to_string());
    emojis.insert("two_hearts".to_string(), "💕".to_string());
    emojis.insert("revolving_hearts".to_string(), "💞".to_string());
    emojis.insert("heartbeat".to_string(), "💓".to_string());
    emojis.insert("heartpulse".to_string(), "💗".to_string());
    emojis.insert("sparkling_heart".to_string(), "💖".to_string());
    emojis.insert("cupid".to_string(), "💘".to_string());
    emojis.insert("gift_heart".to_string(), "💝".to_string());
    
    // Animals
    emojis.insert("dog".to_string(), "🐶".to_string());
    emojis.insert("cat".to_string(), "🐱".to_string());
    emojis.insert("mouse".to_string(), "🐭".to_string());
    emojis.insert("hamster".to_string(), "🐹".to_string());
    emojis.insert("rabbit".to_string(), "🐰".to_string());
    emojis.insert("fox_face".to_string(), "🦊".to_string());
    emojis.insert("bear".to_string(), "🐻".to_string());
    emojis.insert("panda_face".to_string(), "🐼".to_string());
    emojis.insert("koala".to_string(), "🐨".to_string());
    emojis.insert("tiger".to_string(), "🐯".to_string());
    emojis.insert("lion".to_string(), "🦁".to_string());
    emojis.insert("cow".to_string(), "🐮".to_string());
    emojis.insert("pig".to_string(), "🐷".to_string());
    emojis.insert("frog".to_string(), "🐸".to_string());
    emojis.insert("monkey_face".to_string(), "🐵".to_string());
    emojis.insert("see_no_evil".to_string(), "🙈".to_string());
    emojis.insert("hear_no_evil".to_string(), "🙉".to_string());
    emojis.insert("speak_no_evil".to_string(), "🙊".to_string());
    emojis.insert("chicken".to_string(), "🐔".to_string());
    emojis.insert("penguin".to_string(), "🐧".to_string());
    emojis.insert("bird".to_string(), "🐦".to_string());
    emojis.insert("snake".to_string(), "🐍".to_string());
    emojis.insert("turtle".to_string(), "🐢".to_string());
    emojis.insert("bug".to_string(), "🐛".to_string());
    emojis.insert("bee".to_string(), "🐝".to_string());
    emojis.insert("ant".to_string(), "🐜".to_string());
    emojis.insert("lady_beetle".to_string(), "🐞".to_string());
    emojis.insert("spider".to_string(), "🕷️".to_string());
    emojis.insert("fish".to_string(), "🐟".to_string());
    emojis.insert("dolphin".to_string(), "🐬".to_string());
    emojis.insert("whale".to_string(), "🐳".to_string());
    emojis.insert("octopus".to_string(), "🐙".to_string());
    emojis.insert("seahorse".to_string(), "🦄".to_string());
    emojis.insert("dragon".to_string(), "🐉".to_string());
    emojis.insert("unicorn".to_string(), "🦄".to_string());
    
    // Food & Drink
    emojis.insert("apple".to_string(), "🍎".to_string());
    emojis.insert("green_apple".to_string(), "🍏".to_string());
    emojis.insert("pear".to_string(), "🍐".to_string());
    emojis.insert("tangerine".to_string(), "🍊".to_string());
    emojis.insert("lemon".to_string(), "🍋".to_string());
    emojis.insert("banana".to_string(), "🍌".to_string());
    emojis.insert("watermelon".to_string(), "🍉".to_string());
    emojis.insert("grapes".to_string(), "🍇".to_string());
    emojis.insert("strawberry".to_string(), "🍓".to_string());
    emojis.insert("cherries".to_string(), "🍒".to_string());
    emojis.insert("peach".to_string(), "🍑".to_string());
    emojis.insert("pineapple".to_string(), "🍍".to_string());
    emojis.insert("avocado".to_string(), "🥑".to_string());
    emojis.insert("tomato".to_string(), "🍅".to_string());
    emojis.insert("eggplant".to_string(), "🍆".to_string());
    emojis.insert("carrot".to_string(), "🥕".to_string());
    emojis.insert("corn".to_string(), "🌽".to_string());
    emojis.insert("hot_pepper".to_string(), "🌶️".to_string());
    emojis.insert("pizza".to_string(), "🍕".to_string());
    emojis.insert("hamburger".to_string(), "🍔".to_string());
    emojis.insert("fries".to_string(), "🍟".to_string());
    emojis.insert("hotdog".to_string(), "🌭".to_string());
    emojis.insert("taco".to_string(), "🌮".to_string());
    emojis.insert("burrito".to_string(), "🌯".to_string());
    emojis.insert("bread".to_string(), "🍞".to_string());
    emojis.insert("egg".to_string(), "🥚".to_string());
    emojis.insert("pancakes".to_string(), "🥞".to_string());
    emojis.insert("bacon".to_string(), "🥓".to_string());
    emojis.insert("steak".to_string(), "🥩".to_string());
    emojis.insert("poultry_leg".to_string(), "🍗".to_string());
    emojis.insert("spaghetti".to_string(), "🍝".to_string());
    emojis.insert("ramen".to_string(), "🍜".to_string());
    emojis.insert("sushi".to_string(), "🍣".to_string());
    emojis.insert("icecream".to_string(), "🍦".to_string());
    emojis.insert("cake".to_string(), "🍰".to_string());
    emojis.insert("birthday".to_string(), "🎂".to_string());
    emojis.insert("cookie".to_string(), "🍪".to_string());
    emojis.insert("doughnut".to_string(), "🍩".to_string());
    emojis.insert("candy".to_string(), "🍬".to_string());
    emojis.insert("lollipop".to_string(), "🍭".to_string());
    emojis.insert("chocolate_bar".to_string(), "🍫".to_string());
    emojis.insert("coffee".to_string(), "☕".to_string());
    emojis.insert("tea".to_string(), "🍵".to_string());
    emojis.insert("sake".to_string(), "🍶".to_string());
    emojis.insert("beer".to_string(), "🍺".to_string());
    emojis.insert("beers".to_string(), "🍻".to_string());
    emojis.insert("wine_glass".to_string(), "🍷".to_string());
    emojis.insert("cocktail".to_string(), "🍸".to_string());
    emojis.insert("tropical_drink".to_string(), "🍹".to_string());
    
    // Activities
    emojis.insert("soccer".to_string(), "⚽".to_string());
    emojis.insert("basketball".to_string(), "🏀".to_string());
    emojis.insert("football".to_string(), "🏈".to_string());
    emojis.insert("baseball".to_string(), "⚾".to_string());
    emojis.insert("tennis".to_string(), "🎾".to_string());
    emojis.insert("volleyball".to_string(), "🏐".to_string());
    emojis.insert("8ball".to_string(), "🎱".to_string());
    emojis.insert("golf".to_string(), "⛳".to_string());
    emojis.insert("dart".to_string(), "🎯".to_string());
    emojis.insert("video_game".to_string(), "🎮".to_string());
    emojis.insert("game_die".to_string(), "🎲".to_string());
    emojis.insert("musical_note".to_string(), "🎵".to_string());
    emojis.insert("notes".to_string(), "🎶".to_string());
    emojis.insert("art".to_string(), "🎨".to_string());
    
    // Travel & Places
    emojis.insert("car".to_string(), "🚗".to_string());
    emojis.insert("taxi".to_string(), "🚕".to_string());
    emojis.insert("bus".to_string(), "🚌".to_string());
    emojis.insert("train".to_string(), "🚃".to_string());
    emojis.insert("airplane".to_string(), "✈️".to_string());
    emojis.insert("rocket".to_string(), "🚀".to_string());
    emojis.insert("bike".to_string(), "🚲".to_string());
    emojis.insert("anchor".to_string(), "⚓".to_string());
    emojis.insert("mountain".to_string(), "⛰️".to_string());
    emojis.insert("camping".to_string(), "🏕️".to_string());
    emojis.insert("beach_with_umbrella".to_string(), "🏖️".to_string());
    emojis.insert("desert".to_string(), "🏜️".to_string());
    emojis.insert("house".to_string(), "🏠".to_string());
    emojis.insert("office".to_string(), "🏢".to_string());
    emojis.insert("hospital".to_string(), "🏥".to_string());
    emojis.insert("school".to_string(), "🏫".to_string());
    emojis.insert("castle".to_string(), "🏰".to_string());
    
    // Objects
    emojis.insert("watch".to_string(), "⌚".to_string());
    emojis.insert("iphone".to_string(), "📱".to_string());
    emojis.insert("computer".to_string(), "💻".to_string());
    emojis.insert("keyboard".to_string(), "⌨️".to_string());
    emojis.insert("camera".to_string(), "📷".to_string());
    emojis.insert("bulb".to_string(), "💡".to_string());
    emojis.insert("book".to_string(), "📖".to_string());
    emojis.insert("books".to_string(), "📚".to_string());
    emojis.insert("pencil".to_string(), "✏️".to_string());
    emojis.insert("pencil2".to_string(), "✏️".to_string());
    emojis.insert("pen".to_string(), "🖊️".to_string());
    emojis.insert("memo".to_string(), "📝".to_string());
    emojis.insert("briefcase".to_string(), "💼".to_string());
    emojis.insert("scissors".to_string(), "✂️".to_string());
    emojis.insert("lock".to_string(), "🔒".to_string());
    emojis.insert("unlock".to_string(), "🔓".to_string());
    emojis.insert("key".to_string(), "🔑".to_string());
    emojis.insert("hammer".to_string(), "🔨".to_string());
    emojis.insert("wrench".to_string(), "🔧".to_string());
    emojis.insert("gear".to_string(), "⚙️".to_string());
    emojis.insert("bomb".to_string(), "💣".to_string());
    emojis.insert("gun".to_string(), "🔫".to_string());
    emojis.insert("moneybag".to_string(), "💰".to_string());
    emojis.insert("credit_card".to_string(), "💳".to_string());
    emojis.insert("gem".to_string(), "💎".to_string());
    
    // Symbols
    emojis.insert("crown".to_string(), "👑".to_string());
    emojis.insert("trophy".to_string(), "🏆".to_string());
    emojis.insert("medal".to_string(), "🏅".to_string());
    emojis.insert("fire".to_string(), "🔥".to_string());
    emojis.insert("boom".to_string(), "💥".to_string());
    emojis.insert("star".to_string(), "⭐".to_string());
    emojis.insert("star2".to_string(), "🌟".to_string());
    emojis.insert("sparkles".to_string(), "✨".to_string());
    emojis.insert("zap".to_string(), "⚡".to_string());
    emojis.insert("snowflake".to_string(), "❄️".to_string());
    emojis.insert("cloud".to_string(), "☁️".to_string());
    emojis.insert("sunny".to_string(), "☀️".to_string());
    emojis.insert("rainbow".to_string(), "🌈".to_string());
    emojis.insert("umbrella".to_string(), "☔".to_string());
    emojis.insert("tada".to_string(), "🎉".to_string());
    emojis.insert("confetti_ball".to_string(), "🎊".to_string());
    emojis.insert("balloon".to_string(), "🎈".to_string());
    emojis.insert("gift".to_string(), "🎁".to_string());
    emojis.insert("checkered_flag".to_string(), "🏁".to_string());
    emojis.insert("triangular_flag_on_post".to_string(), "🚩".to_string());
    emojis.insert("white_check_mark".to_string(), "✅".to_string());
    emojis.insert("x".to_string(), "❌".to_string());
    emojis.insert("bangbang".to_string(), "‼️".to_string());
    emojis.insert("question".to_string(), "❓".to_string());
    emojis.insert("exclamation".to_string(), "❗".to_string());
    emojis.insert("warning".to_string(), "⚠️".to_string());
    emojis.insert("no_entry".to_string(), "⛔".to_string());
    emojis.insert("100".to_string(), "💯".to_string());
    emojis.insert("sos".to_string(), "🆘".to_string());
    emojis.insert("cool".to_string(), "🆒".to_string());
    emojis.insert("free".to_string(), "🆓".to_string());
    emojis.insert("new".to_string(), "🆕".to_string());
    emojis.insert("up".to_string(), "🆙".to_string());
    emojis.insert("ok".to_string(), "🆗".to_string());
    
    // Arrows & Symbols
    emojis.insert("arrow_up".to_string(), "⬆️".to_string());
    emojis.insert("arrow_down".to_string(), "⬇️".to_string());
    emojis.insert("arrow_left".to_string(), "⬅️".to_string());
    emojis.insert("arrow_right".to_string(), "➡️".to_string());
    emojis.insert("arrow_upper_right".to_string(), "↗️".to_string());
    emojis.insert("arrow_lower_right".to_string(), "↘️".to_string());
    emojis.insert("arrow_lower_left".to_string(), "↙️".to_string());
    emojis.insert("arrow_upper_left".to_string(), "↖️".to_string());
    emojis.insert("rewind".to_string(), "⏪".to_string());
    emojis.insert("fast_forward".to_string(), "⏩".to_string());
    emojis.insert("arrow_double_up".to_string(), "⏫".to_string());
    emojis.insert("arrow_double_down".to_string(), "⏬".to_string());
    emojis.insert("recycle".to_string(), "♻️".to_string());
    emojis.insert("infinity".to_string(), "♾️".to_string());
    
    emojis
}

// Search for emojis by name with similarity scoring
pub fn search_emojis_by_name(emoji_map: &HashMap<String, String>, query: &str) -> Vec<(String, String, i32)> {
    let mut results: Vec<(String, String, i32)> = emoji_map
        .iter()
        .map(|(name, emoji)| {
            let score = calculate_similarity(name, query);
            (name.clone(), emoji.clone(), score)
        })
        .collect();
    
    // Sort by score (descending)
    results.sort_by(|a, b| b.2.cmp(&a.2));
    
    results
}

fn calculate_similarity(str1: &str, str2: &str) -> i32 {
    let str1_lower = str1.to_lowercase();
    let str2_lower = str2.to_lowercase();

    // Exact match gets highest score
    if str1_lower == str2_lower {
        return 1000;
    }

    // Starts with query gets high score
    if str1_lower.starts_with(&str2_lower) {
        return 800;
    }

    // Contains query gets medium score
    if str1_lower.contains(&str2_lower) {
        return 600;
    }

    // Levenshtein distance for fuzzy matching
    let len1 = str1_lower.len();
    let len2 = str2_lower.len();
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    // Initialize matrix
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    // Fill matrix
    let str1_chars: Vec<char> = str1_lower.chars().collect();
    let str2_chars: Vec<char> = str2_lower.chars().collect();
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if str1_chars[i - 1] == str2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = *[
                matrix[i - 1][j] + 1,      // deletion
                matrix[i][j - 1] + 1,      // insertion
                matrix[i - 1][j - 1] + cost // substitution
            ].iter().min().unwrap();
        }
    }

    let distance = matrix[len1][len2];
    
    // Convert distance to similarity score (0-400 range)
    let similarity = 400 - (distance * 50) as i32;
    similarity.max(0)
}