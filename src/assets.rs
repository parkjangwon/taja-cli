pub fn game_words(is_korean: bool) -> &'static [&'static str] {
    if is_korean {
        &[
            "하늘", "바다", "나무", "구름", "태양", "바람", "사람", "우주", "도시", "강물",
            "노래", "마음", "봄날", "여름", "가을", "겨울", "행복", "사랑", "미래", "우정",
            "아침", "저녁", "시간", "기억", "평화", "희망", "노력", "열정", "도전", "성공",
            "세계", "지도", "기쁨", "슬픔", "위로", "용기", "감사", "약속", "여행", "예술",
            "소리", "향기", "미소", "눈물", "바위", "모래", "조개", "낙엽", "학교", "친구",
            "공부", "연필", "운동장", "농구", "축구", "야구", "영화", "달빛", "별빛", "파도",
            "갈매기", "고래", "상어", "돌고래", "캠핑", "텐트", "배낭", "토끼", "사슴", "나비",
            "꽃잎", "정원", "공원", "산책", "자전거", "기차", "비행기", "항구", "역사", "그림",
            "음악", "피아노", "기타", "축제", "추억", "소원", "보물", "모험", "자유", "지혜",
            "건강", "웃음", "응원", "승리", "가족", "이웃", "마을", "김치", "비빔밥", "떡볶이",
            "라면", "초밥", "커피", "녹차", "사과", "딸기", "포도", "수박", "바나나", "망고",
        ]
    } else {
        &[
            "apple", "banana", "cherry", "orange", "grape", "melon", "peach", "lemon", "lime",
            "computer", "keyboard", "monitor", "screen", "network", "system", "database",
            "science", "history", "english", "travel", "nature", "forest", "mountain",
            "summer", "winter", "spring", "weather", "yellow", "purple", "golden",
            "sunshine", "galaxy", "universe", "planet", "ocean", "island", "beach",
            "dolphin", "whale", "campfire", "backpack", "rabbit", "tiger", "butterfly",
            "garden", "bicycle", "train", "airplane", "library", "painting", "music",
            "piano", "guitar", "concert", "festival", "memory", "dream", "treasure",
            "adventure", "miracle", "freedom", "courage", "wisdom", "knowledge",
            "happiness", "fortune", "victory", "gratitude", "family", "teacher", "student",
            "doctor", "artist", "writer", "camera", "phone", "internet", "email", "letter",
        ]
    }
}

pub fn daily_seed_words(is_korean: bool) -> &'static [&'static str] {
    if is_korean {
        &[
            "하늘", "바다", "나무", "구름", "태양", "바람", "사람", "우주", "도시", "강물",
            "노래", "마음", "봄날", "여름", "가을", "겨울", "행복", "사랑", "미래", "우정",
            "아침", "저녁", "시간", "기억", "평화", "희망", "노력", "열정", "도전", "성공",
            "세계", "기쁨", "용기", "감사", "약속", "여행", "예술", "소리", "향기", "미소",
            "학교", "친구", "공부", "연필", "축구", "야구", "영화", "달빛", "별빛", "파도",
        ]
    } else {
        &[
            "apple", "banana", "cherry", "ocean", "galaxy", "planet", "mountain", "forest",
            "butterfly", "dolphin", "treasure", "adventure", "freedom", "courage", "wisdom",
            "knowledge", "happiness", "sunshine", "universe", "miracle", "guitar", "painting",
            "library", "concert", "festival", "memory", "garden", "bicycle", "victory",
            "student", "teacher", "doctor", "artist", "camera", "internet", "digital",
            "science", "history", "english", "nature", "summer", "winter", "spring", "weather",
            "golden", "silver", "island", "dream", "letter", "story",
        ]
    }
}

pub fn long_text_titles(is_korean: bool) -> &'static [&'static str] {
    if is_korean {
        &[
            "1. 애국가 (1~4절)",
            "2. 훈민정음 어제 서문",
            "3. 진달래꽃 (김소월)",
            "4. 대한민국 헌법 제1장",
        ]
    } else {
        &[
            "1. Gettysburg Address (아브라함 링컨)",
            "2. I Have a Dream (마틴 루터 킹)",
            "3. The Road Not Taken (로버트 프로스트)",
        ]
    }
}

pub fn finger_practice_words(level: usize, is_korean: bool) -> &'static [&'static str] {
    if is_korean {
        match level {
            1 => &["ㅁㄴㅇㄹ", "ㅓㅏㅣ;", "ㄹㅇㄴㅁ", "ㅓㅏㅣ;", "ㅁㄴㅇㄹㅓㅏㅣ;", "ㅇㄹㄴㅁㅣㅏㅓ;"],
            2 => &["ㅎㅗㅜ", "ㅁㄴㅇㄹㅎ", "ㅓㅏㅣ;ㅗㅜ", "하하", "호호", "우우", "나나", "다다", "로로"],
            3 => &["ㅂㅈㄷㄱㅅ", "ㅛㅕㅑㅐㅔ", "ㅂㄱㄷㅈㅅ", "ㅕㅛㅐㅑㅔ", "가방", "사자", "도끼", "벼루", "새조개"],
            4 => &["ㅋㅌㅊㅍ", "ㅠㅜㅡ", "ㅋㅍㅌㅊ", "ㅡㅠㅜ", "카드", "파도", "코트", "토끼", "참새", "하늘"],
            5 => &["12345", "67890", "-=[]\\", ";'", ",./", "102938", "4756", "sys.argv[0]", "1 + 2 = 3"],
            6 => &["ㅃㅉㄸㄲㅆ", "ㅒㅖ", "아빠", "짜장면", "떡꼬치", "쓰레기", "꼬마", "얘기", "계란"],
            _ => &["ㅁㄴㅇㄹ"],
        }
    } else {
        match level {
            1 => &["asdf", "jkl;", "fdsa", "jkl;", "ask", "dad", "lad", "salad", "sad", "fall", "flask"],
            2 => &["qwer", "uiop", "quiet", "power", "write", "wire", "peak", "keep", "peer", "route", "trip"],
            3 => &["zxcv", "m,./", "zoom", "cave", "zero", "move", "class", "voice", "music", "box", "zone"],
            4 => &["Hello", "World", "Rust", "Linux", "Code", "Type", "System", "Dynamic", "Static", "Gemini"],
            5 => &["12345", "67890", "!@#$", "%^&*", "()_+", "{}|", ":\"<>?", "[]\\;',./"],
            _ => &["asdf"],
        }
    }
}

pub fn word_practice_words(is_korean: bool) -> &'static [&'static str] {
    if is_korean {
        &[
            "하늘", "바다", "나무", "구름", "태양", "바람", "사람", "우주", "도시", "강물",
            "노래", "마음", "봄날", "여름", "가을", "겨울", "행복", "사랑", "미래", "우정",
            "아침", "저녁", "시간", "기억", "평화", "희망", "노력", "열정", "도전", "성공",
            "세계", "지도", "기쁨", "슬픔", "위로", "용기", "감사", "약속", "여행", "예술",
            "소리", "빛깔", "향기", "미소", "눈물", "길목", "바위", "모래", "조개", "낙엽",
            "학교", "친구", "공부", "책장", "연필", "노트", "교실", "운동장", "농구", "축구",
            "야구", "취미", "영화", "소설", "시인", "낭만", "달빛", "별빛", "은하수", "성운",
            "태양계", "행성", "혜성", "지구", "대륙", "섬나라", "파도", "수평선", "갈매기", "고래",
            "상어", "돌고래", "물고기", "산호초", "조약돌", "모닥불", "캠핑", "텐트", "배낭", "숲길",
            "다람쥐", "토끼", "사슴", "호랑이", "독수리", "비둘기", "참새", "제비", "나비", "벌꿀",
            "꽃잎", "정원", "공원", "분수", "벤치", "산책", "자전거", "자동차", "기차", "비행기",
            "돛단배", "항구", "선장", "지도자", "영웅", "역사", "유적", "박물관", "도서관", "책방",
            "그림", "조각", "음악", "피아노", "바이올린", "드럼", "기타", "노래방", "콘서트", "연극",
            "뮤지컬", "축제", "불꽃놀이", "명절", "추억", "일기", "소원", "꿈나라", "천사", "동화",
            "신화", "전설", "보물", "모험", "신비", "기적", "환상", "현실", "진실", "정의",
            "자유", "평등", "협동", "나눔", "배려", "이해", "용서", "화해", "존중", "신뢰",
            "성실", "책임", "지혜", "지식", "생각", "사색", "명상", "휴식", "건강", "웃음",
            "행운", "환희", "설렘", "고독", "분노", "극복", "달성", "보람", "만족", "축하",
            "응원", "격려", "칭찬", "승리", "아기", "가족", "부모", "형제", "이웃", "마을",
            "사자", "코끼리", "기린", "여우", "늑대", "원숭이", "팬더", "펭귄", "부엉이", "까마귀",
            "두루미", "오리", "거위", "백조", "앵무새", "개구리", "거북이", "악어", "도마뱀", "달팽이",
            "잠자리", "매미", "무당벌레", "사마귀", "장수풍뎅이", "반딧불이", "장미", "튤립", "백합", "해바라기",
            "무궁화", "벚꽃", "진달래", "개나리", "단풍나무", "은행나무", "소나무", "대나무", "버드나무", "코스모스",
            "민들레", "나팔꽃", "연꽃", "선인장", "안개꽃", "카네이션", "국화", "라벤더", "허브", "이끼",
            "밥상", "김치", "비빔밥", "불고기", "삼겹살", "갈비", "냉면", "떡볶이", "순대", "튀김",
            "라면", "국수", "짜장면", "짬뽕", "탕수육", "만두", "피자", "햄버거", "치킨", "샐러드",
            "샌드위치", "스파게티", "스테이크", "초밥", "우동", "라멘", "돈가스", "카레", "빵집", "과자",
            "초콜릿", "사탕", "젤리", "아이스크림", "커피", "녹차", "홍차", "우유", "두유", "주스",
            "사과", "배", "귤", "감", "밤", "대추", "호두", "땅콩", "아몬드", "수박",
            "참외", "멜론", "딸기", "포도", "복숭아", "자두", "살구", "체리", "토마토", "바나나",
            "파인애플", "망고", "키위", "블루베리", "레몬", "오렌지", "자몽", "코코넛", "무화과", "석류",
            "모자", "안경", "셔츠", "바지", "치마", "원피스", "코트", "패딩", "양말", "신발",
            "구두", "운동화", "슬리퍼", "장갑", "목도리", "우산", "가방", "지갑", "시계", "벨트",
            "침대", "이불", "베개", "책상", "의자", "옷장", "서랍장", "화장대", "책장", "소파",
            "식탁", "싱크대", "냉장고", "세탁기", "건조기", "청소기", "에어컨", "선풍기", "텔레비전", "컴퓨터",
            "노트북", "키보드", "마우스", "스피커", "이어폰", "헤드폰", "스마트폰", "태블릿", "카메라", "렌즈",
            "도서관", "미술관", "박물관", "체육관", "수영장", "영화관", "공연장", "경기장", "놀이공원", "동물원",
            "식물원", "백화점", "마트", "시장", "약국", "병원", "은행", "우체국", "소방서", "경찰서",
            "구청", "시청", "법원", "대학교", "고등학교", "중학교", "초등학교", "유치원", "학원", "독서실",
            "버스", "택시", "지하철", "열차", "고속열차", "오토바이", "킥보드", "헬리콥터", "전투기", "우주선",
            "인공위성", "로켓", "잠수함", "크루즈선", "화물선", "소방차", "경찰차", "구급차", "포클레인", "트럭",
            "엔진", "모터", "배터리", "발전소", "태양광", "풍력", "수력", "원자력", "전기", "전자",
            "반도체", "인공지능", "로봇", "드론", "네트워크", "인터넷", "소프트웨어", "프로그램", "알고리즘", "데이터",
            "수학", "물리학", "화학", "생물학", "지구과학", "천문학", "의학", "약학", "공학", "컴퓨터공학",
            "국어", "영어", "역사학", "지리학", "철학", "심리학", "사회학", "정치학", "경제학", "경영학",
            "법학", "행정학", "교육학", "체육학", "예술학", "디자인", "건축학", "고고학", "인류학", "통계학",
            "교사", "교수", "의사", "간호사", "약사", "수의사", "변호사", "판사", "검사", "변리사",
            "회계사", "세무사", "건축가", "디자이너", "화가", "조각가", "음악가", "작곡가", "지휘자", "가수",
            "배우", "감독", "작가", "시인", "소설가", "기자", "아나운서", "프로듀서", "엔지니어", "개발자",
            "프로그래머", "기획자", "연구원", "과학자", "요리사", "제빵사", "바리스타", "소믈리에", "승무원", "조종사",
            "군인", "경찰관", "소방관", "공무원", "정치인", "외교관", "통역사", "번역가", "농부", "어부",
            "즐거움", "신바람", "두근두근", "설레임", "안도감", "평온함", "유쾌함", "상쾌함", "상상력", "창의성",
            "자긍심", "성취감", "자신감", "열정적", "헌신적", "도전 정신", "모험심", "호기심", "동정심", "애국심",
            "시민 의식", "도덕성", "투명성", "공정함", "정직함", "겸손함", "배려심", "협동심", "인내심", "자제력",
            "집중력", "기억력", "이해력", "창조적", "혁신적", "합리적", "논리적", "과학적", "예술적", "대중적",
            "전통적", "현대적", "글로벌", "로컬", "자연 친화", "환경 보호", "지속 가능", "다양성", "포용성", "개방성",
            "행복감", "안정감", "친밀감", "신뢰도", "투명도", "만족도", "기여도", "참여도", "성장통", "희망 사항",
            "가위", "풀", "테이프", "클립", "자석", "핀", "압정", "칠판", "분필", "지우개",
            "형광펜", "볼펜", "샤프", "연필깎이", "필통", "자", "각도기", "컴퍼스", "돋보기", "현미경",
            "망원경", "나침반", "지도", "지구본", "저울", "온도계", "기압계", "습도계", "시계바늘", "초침",
            "거울", "유리창", "커튼", "블라인드", "카펫", "매트", "쿠션", "방석", "벽지", "장판",
            "천장", "기둥", "대문", "창문", "현관", "베란다", "보일러", "라디에이터", "난로", "벽난로",
            "가스레인지", "인덕션", "전자레인지", "식기세척기", "토스터", "믹서기", "전기포트", "압력밥솥", "냄비", "프라이팬",
            "칼", "도마", "국자", "뒤집개", "집게", "수저", "젓가락", "숟가락", "포크", "나이프",
            "접시", "대접", "공기", "종지", "컵", "머그잔", "텀블러", "물병", "찬장", "식탁보",
            "수건", "비누", "샴푸", "린스", "바디워시", "치약", "칫솔", "면도기", "화장지", "물티슈",
            "빗", "헤어드라이어", "고데기", "화장품", "로션", "스킨", "선크림", "향수", "손톱깎이", "귀이개",
            "바늘", "실", "단추", "지퍼", "가위질", "바느질", "재봉틀", "다리미", "다리미판", "빨래집게",
            "빨래건조대", "옷걸이", "수납함", "상자", "바구니", "비닐봉지", "쓰레기통", "분리수거함", "빗자루", "쓰레받기",
            "먼지털이", "걸레", "대걸레", "양동이", "호스", "샤워기", "수도꼭지", "배수구", "환풍기", "도어락",
            "열쇠", "자물쇠", "체인", "로프", "끈", "고무줄", "테이프", "본드", "풀칠", "페인트",
            "붓", "롤러", "사포", "망치", "못", "나사", "드라이버", "스패너", "펜치", "니퍼",
            "톱", "대패", "송곳", "줄자", "수평계", "사다리", "리어카", "수레", "지게차", "크레인",
            "기쁨", "즐거움", "유쾌", "통쾌", "상쾌", "명랑", "활기", "생기", "용기", "자신감",
            "기대", "설렘", "희망", "바람", "소망", "소원", "꿈", "이상", "동경", "그리움",
            "사랑", "자비", "은혜", "감사", "고마움", "만족", "보람", "긍지", "자부심", "안도",
            "평온", "안정", "여유", "휴식", "위안", "위로", "동정", "연민", "이해", "용서",
            "화해", "협동", "우정", "신뢰", "믿음", "의리", "충성", "효도", "공경", "배려",
            "양보", "친절", "온정", "따뜻함", "순수", "솔직", "정직", "진실", "성실", "근면",
            "인내", "끈기", "노력", "열정", "집념", "의지", "지혜", "슬기", "재치", "유머",
            "독창성", "개성", "매력", "아름다움", "우아함", "세련됨", "자연스러움", "평범함", "특별함", "소중함",
            "영원함", "순간", "추억", "기억", "흔적", "발자국", "그림자", "메아리", "울림", "기적",
        ]
    } else {
        &[
            "apple", "banana", "cherry", "orange", "grape", "melon", "peach", "berry", "lemon", "lime",
            "computer", "keyboard", "monitor", "mouse", "screen", "terminal", "console", "network", "system", "database",
            "science", "history", "subject", "object", "english", "korean", "travel", "nature", "forest", "mountain",
            "summer", "winter", "spring", "autumn", "weather", "yellow", "purple", "orange", "silver", "golden",
            "sunshine", "starlight", "moonlight", "galaxy", "universe", "planet", "earth", "ocean", "island", "beach",
            "dolphin", "whale", "seagull", "pebble", "campfire", "camping", "backpack", "pathway", "squirrel", "rabbit",
            "tiger", "eagle", "butterfly", "flower", "garden", "park", "fountain", "bicycle", "vehicle", "train",
            "airplane", "harbor", "captain", "leader", "history", "museum", "library", "bookstore", "painting", "sculpture",
            "music", "piano", "violin", "guitar", "concert", "theater", "festival", "fireworks", "memory", "diary",
            "wish", "dream", "angel", "fairytale", "myth", "legend", "treasure", "adventure", "mystery", "miracle",
            "fantasy", "reality", "truth", "justice", "freedom", "equality", "peace", "sharing", "respect", "trust",
            "wisdom", "knowledge", "thinking", "meditation", "relax", "health", "laughter", "happiness", "fortune", "excitement",
            "solitude", "sadness", "anger", "victory", "reward", "satisfaction", "gratitude", "welcome", "support", "courage",
            "helper", "partner", "colleague", "family", "parent", "sibling", "teacher", "student", "doctor", "engineer",
            "artist", "writer", "poet", "novel", "story", "drama", "stage", "actors", "camera", "picture", "album",
            "photo", "frame", "mirror", "window", "door", "house", "room", "kitchen", "table", "chair", "clock",
            "phone", "internet", "website", "email", "message", "letter", "stamp", "paper", "book", "pen", "pencil",
            "lion", "elephant", "giraffe", "fox", "wolf", "monkey", "panda", "penguin", "owl", "crow",
            "swan", "duck", "goose", "parrot", "frog", "turtle", "crocodile", "lizard", "snail", "dragonfly",
            "beetle", "butterfly", "rose", "tulip", "lily", "sunflower", "maple", "pine", "bamboo", "willow",
            "cosmos", "dandelion", "cactus", "orchid", "carnation", "lavender", "rosemary", "basil", "mint", "moss",
            "star", "nebula", "comet", "asteroid", "orbit", "gravity", "eclipse", "mercury", "venus", "mars",
            "jupiter", "saturn", "uranus", "neptune", "pluto", "crater", "telescope", "astronaut", "satellite", "galaxy",
            "bread", "butter", "cheese", "yogurt", "cream", "ice", "juice", "water", "tea", "coffee",
            "chocolate", "candy", "cookie", "cake", "pie", "donut", "pizza", "burger", "pasta", "salad",
            "soup", "steak", "sushi", "rice", "noodle", "curry", "stew", "sauce", "spices", "pepper",
            "salt", "sugar", "honey", "onion", "garlic", "potato", "carrot", "tomato", "cucumber", "cabbage",
            "spinach", "broccoli", "mushroom", "pumpkin", "bean", "corn", "nut", "fruit", "berry", "grape",
            "hat", "glasses", "shirt", "pants", "skirt", "dress", "coat", "jacket", "socks", "shoes",
            "boots", "sneakers", "slippers", "gloves", "scarf", "umbrella", "bag", "wallet", "watch", "belt",
            "bed", "blanket", "pillow", "desk", "chair", "closet", "drawer", "sofa", "table", "cabinet",
            "fridge", "dryer", "cleaner", "aircon", "fan", "tv", "laptop", "tablet", "phone", "camera",
            "battery", "charger", "cable", "plug", "socket", "switch", "lamp", "bulb", "mirror", "window",
            "city", "town", "village", "street", "road", "highway", "bridge", "tunnel", "station", "airport",
            "harbor", "port", "office", "factory", "store", "shop", "market", "mall", "bank", "hospital",
            "pharmacy", "clinic", "school", "college", "academy", "museum", "gallery", "library", "theater", "cinema",
            "park", "square", "court", "senate", "embassy", "police", "firehouse", "post", "hotel", "restaurant",
            "math", "physics", "chemistry", "biology", "geology", "astronomy", "medicine", "pharmacy", "engineering", "coding",
            "history", "geography", "philosophy", "psychology", "sociology", "politics", "economy", "law", "education", "design",
            "engine", "motor", "robot", "drone", "network", "internet", "software", "program", "algorithm", "data",
            "silicon", "hardware", "firmware", "sensor", "laser", "radar", "sonar", "optics", "acoustics", "statics",
            "scissor", "glue", "tape", "clip", "magnet", "pin", "chalk", "eraser", "pencil", "ruler",
            "compass", "scale", "sensor", "lens", "glass", "window", "door", "wall", "floor", "ceiling",
            "roof", "gate", "fence", "yard", "garden", "flowerpot", "fountain", "bench", "path", "streetlamp",
            "kettle", "teapot", "cup", "mug", "glass", "bottle", "plate", "bowl", "spoon", "fork",
            "knife", "napkin", "tablecloth", "tray", "apron", "pot", "pan", "oven", "grill", "toaster",
            "soap", "shampoo", "toothpaste", "toothbrush", "razor", "towel", "comb", "dryer", "perfume", "lotion",
            "needle", "thread", "button", "zipper", "sewing", "iron", "hanger", "box", "basket", "bag",
            "broom", "dustpan", "mop", "bucket", "hose", "shower", "tap", "valve", "lock", "key",
            "chain", "rope", "string", "wire", "cable", "screw", "bolt", "nut", "nail", "hammer",
            "saw", "drill", "file", "ruler", "level", "ladder", "wagon", "cart", "crane", "loader",
            "joy", "pleasure", "delight", "glee", "bliss", "cheer", "vigor", "courage", "boldness", "trust",
            "hope", "wish", "desire", "dream", "vision", "aspiration", "nostalgia", "longing", "memory", "trace",
            "love", "mercy", "grace", "gratitude", "thanks", "satisfaction", "pride", "glory", "honor", "peace",
            "calm", "serenity", "rest", "relief", "comfort", "sympathy", "pity", "empathy", "pardon", "mercy",
            "harmony", "friendship", "loyalty", "faith", "devotion", "piety", "respect", "care", "warmth", "kindness",
            "honesty", "truth", "sincerity", "diligence", "patience", "grit", "passion", "willpower", "wisdom", "wits",
            "humor", "talent", "genius", "charm", "beauty", "elegance", "grace", "novelty", "miracle", "wonder",
            "moment", "instant", "decade", "century", "epoch", "era", "future", "past", "present", "eternal",
        ]
    }
}

pub fn sentence_practice_sentences(is_korean: bool) -> &'static [&'static str] {
    if is_korean {
        &[
            "동해 물과 백두산이 마르고 닳도록 하느님이 보우하사 우리나라 만세",
            "남산 위에 저 소나무 철갑을 두른 듯 바람 서리 불변함은 우리 기상일세",
            "가을 하늘 공활한데 높고 구름 없이 밝은 달은 우리 가슴 일편단심일세",
            "이 기상과 이 맘으로 충성을 다하여 괴로우나 즐거우나 나라 사랑하세",
            "나랏말싸미 듕귁에 달아 문자와로 서르 사맛디 아니할쎄 이런 젼차로 어린 백셩이 니르고져 홇배이셔도",
            "아름다운 이 땅에 금수강산에 단군할아버지가 터 잡으시고 홍익인간 뜻으로 나라 세우니 대대손손 훌륭한 인물도 많아",
            "별을 노래하는 마음으로 모든 죽어가는 것들을 사랑해야지 그리고 나한테 주어진 길을 걸어가야겠다",
            "삶이 그대를 속일지라도 슬퍼하거나 노하지 말라 우울한 날들을 견디면 믿으라 기쁨의 날이 오리니",
            "대한민국은 민주공화국이다 대한민국의 주권은 국민에게 있고 모든 권력은 국민으로부터 나온다",
            "가는 말이 고와야 오는 말이 곱다 소 잃고 외양간 고친다 돌다리도 두들겨 보고 건너라",
            "천 리 길도 한 걸음부터 시작된다 시작이 반이다 호랑이도 제 말 하면 온다",
            "벼는 익을수록 고개를 숙인다 아는 길도 물어가라 백지장도 맞들면 낫다",
            "인생은 짧고 예술은 길다 자신을 아는 것이 가장 위대한 지식이다",
            "어둠이 깊을수록 별은 더욱 빛난다 고난 뒤에 오는 낙이 진정한 기쁨이다",
            "오늘 할 일을 내일로 미루지 말라 시간은 아무도 기다려주지 않는다",
            "서로를 존중하고 배려하는 마음이 아름다운 공동체를 만든다",
            "실패는 성공의 어머니이다 포기하지 않는 자에게 기회가 온다",
            "배움에는 끝이 없다 매일 조금씩 성장하는 나를 발견해 보자",
            "가을바람에 흔들리는 코스모스처럼 우리의 마음도 가끔 흔들릴 때가 있다",
            "하늘 높이 날아오르는 새처럼 우리의 꿈도 드넓은 세상을 향해 뻗어가길",
            "봄바람 휘날리며 흩날리는 벚꽃 잎이 울려 퍼질 이 거리를 둘이 걸어요",
            "기회는 노크하지 않는다 당신이 기회의 문을 두드려야 한다",
            "가장 어두운 밤도 언젠가는 끝나고 해는 다시 떠오를 것이다",
            "흔들리지 않고 피는 꽃이 어디 있으랴 이 세상 그 어떤 아름다운 꽃들도 다 흔들리며 피었나니",
            "자세히 보아야 예쁘다 오래 보아야 사랑스럽다 너도 그렇다",
            "네 장미꽃이 그토록 소중한 것은 네가 그 꽃을 위해 잃어버린 시간 때문이다",
            "중요한 것은 꺾이지 않는 마음이다 끝까지 포기하지 마라",
            "시간은 우리가 가진 가장 소중한 자산이며 결코 되돌릴 수 없다",
            "행복은 깊이 생각하는 자의 몫이며 나눌수록 커지는 신비한 보물이다",
            "성공한 사람이 되려 하기보다 가치 있는 사람이 되려고 노력하라",
            "우리가 걷는 이 길이 비록 험난할지라도 끝내 웃으며 맞이할 내일이 있다",
            "작은 변화가 모여 큰 기적을 만든다 오늘부터 조금씩 나아가자",
            "나 자신을 사랑하는 법을 배우는 것이 세상에서 가장 훌륭한 사랑이다",
            "바다를 바라보며 넓은 마음을 배우고 산을 오르며 굳건한 의지를 다진다",
            "따뜻한 말 한마디가 누군가의 인생을 바꾸는 큰 힘이 될 수 있다",
            "바람이 불어오는 곳 그곳으로 가네 그대 머릿결 같은 나무 아래로",
            "행복의 한쪽 문이 닫히면 다른 쪽 문이 열린다 그러나 흔히 우리는 닫힌 문만 바라본다",
            "꿈을 꿀 수 있다면 그 꿈을 이룰 수도 있다 시작하는 용기를 가져라",
            "인간은 생각하는 갈대이다 비록 약하지만 사색을 통해 우주를 담는다",
            "오늘의 고난은 내일의 영광을 위한 밑거름이 될 뿐이다 기죽지 마라",
            "푸른 하늘 은하수 하얀 쪽배에 계수나무 한 나무 토끼 한 마리",
            "강물이 흘러 흘러 바다로 가듯이 우리의 노력도 언젠가 결실을 맺는다",
            "가장 위대한 승리는 타인을 이기는 것이 아니라 자신을 극복하는 것이다",
            "독서는 앉아서 하는 가장 위대한 여행이며 생각의 지평을 넓혀준다",
            "친구는 제2의 자신이다 서로의 아픔을 보듬고 기쁨을 함께 나누자",
            "아침 이슬 머금은 풀잎처럼 우리의 마음도 늘 맑고 싱그럽게 유지되길",
            "인생이라는 도화지에 당신만의 아름다운 색깔로 꿈을 그려 나가라",
            "건강한 육체에 건강한 정신이 깃든다 매일 스스로를 소중히 가꾸자",
            "고요한 숲속의 물소리처럼 마음을 차분히 내려놓고 명상에 잠겨보자",
            "모든 순간이 꽃봉오리인 것을 내 열심에 따라 꽃피어날 것들을",
            "정의는 반드시 승리하며 진실은 가려져도 언젠가 빛을 발하게 된다",
            "지혜로운 사람은 행동으로 증명하고 어리석은 사람은 말로만 과시한다",
            "시간을 아끼는 것은 인생을 사랑하는 법을 실천하는 가장 확실한 길이다",
            "따뜻한 미소 하나가 차가운 세상을 밝히는 등불이 될 수 있다",
            "어떤 분야든 매일 한 시간씩 투자하면 누구나 전문가가 될 수 있다",
            "새벽이 오기 직전이 가장 어둡다 조금만 더 버티면 빛이 올 것이다",
            "배려하는 마음은 향기로운 꽃과 같아서 주변을 모두 아름답게 물들인다",
            "포기하지 않는 집념이야말로 모든 위대한 업적의 공통된 열쇠이다",
            "매일 아침 눈을 뜰 때마다 새로운 기회가 주어짐에 감사하자",
            "당신이 오늘 심은 작은 씨앗이 내일 울창한 숲이 될 것입니다",
            "자연은 서두르지 않지만 모든 것을 이룬다 우리도 조급해하지 말자",
            "사랑은 온전히 자신을 내어주는 것이며 대가를 바라지 않는 신비다",
            "역사를 잊은 민족에게 미래는 없다 선조들의 발자취를 돌아보라",
            "박물관의 오래된 유물 속에서 우리는 과거와 현재의 연결고리를 찾는다",
            "아름다운 시 구절 하나가 가슴 깊이 파고들어 평생의 위로가 된다",
            "피아노의 건반이 조화를 이루듯 우리의 삶도 일과 휴식의 균형이 필요하다",
            "콘서트홀을 가득 메운 열기처럼 우리의 청춘도 뜨겁게 타오르길",
            "어릴 적 읽던 동화책 속 모험은 여전히 우리 가슴속에 살아 숨 쉰다",
            "상상할 수 있는 모든 것은 현실이 될 수 있다 꿈을 제한하지 마라",
            "겸손은 사람을 가장 아름답게 만드는 향기이며 깊은 울림을 준다",
            "지속 가능한 발전을 위해 우리는 자연과 공존하는 길을 찾아야 한다",
            "도전하지 않는 삶은 정체될 뿐이다 날개를 펴고 드넓은 하늘로 날아라",
            "서로 존중하는 마음이 가득할 때 비로소 진정한 평화가 싹튼다",
            "신뢰는 쌓기는 어렵지만 무너지기는 쉽다 매 순간 성실히 임하자",
            "책임감 있는 행동이 성숙한 어른을 만들고 사회를 올바르게 이끈다",
            "오늘 하루도 수고한 나 자신에게 격려와 찬사의 미소를 보내주자",
            "가족의 따뜻한 품은 세상 어떤 거친 풍파도 막아주는 방패가 된다",
            "어부의 그물에 걸린 물고기처럼 우리의 삶도 얽혀 있지만 풀 수 있다",
            "소방관들의 헌신적인 노고 덕분에 우리는 오늘도 안전한 하루를 보낸다",
            "반도체 강국의 위상을 넘어 인공지능 시대의 주역으로 우뚝 서자",
            "소프트웨어 코딩은 논리적인 사색의 결과이며 새로운 세상을 창조한다",
            "매일 쓰는 일기 속에 나의 생각과 성장의 흔적들이 고스란히 담긴다",
            "우리가 꿈꾸는 기적은 멀리 있지 않다 매일의 작은 노력이 기적이다",
            "화가의 화폭에 담긴 찬란한 색채처럼 인생을 아름답게 칠해보자",
            "도서관의 고요한 공기 속에서 책장을 넘기는 소리가 마음을 채운다",
            "가을바람에 떨어지는 낙엽은 끝이 아니라 새로운 시작을 위한 준비다",
            "수평선 너머로 붉게 타오르는 노을을 바라보며 깊은 사색에 잠긴다",
            "우리가 나누는 작은 따뜻함이 모여 세상을 훈훈하게 변화시킬 것이다",
            "실패를 두려워하여 멈추기보다 전진하며 배우는 자세가 훌륭하다",
            "모든 배움은 호기심에서 출발하며 호기심은 우리를 성장하게 만든다",
            "정직함은 가장 확실한 자산이며 인생의 길을 밝히는 나침반이다",
            "지나온 발자취를 돌아보며 현재를 점검하고 더 나은 미래를 설계하자",
            "자전거 페달을 밟으며 스치는 싱그러운 바람을 온몸으로 느껴보자",
            "여객선 창밖으로 펼쳐진 드넓은 바다를 보며 웅장한 꿈을 품는다",
            "매 순간 집중하며 최선을 다하는 태도가 결국 빛나는 성공을 이끈다",
            "따뜻한 온정이 넘치는 이웃들과 함께할 때 삶은 더욱 행복해진다",
            "조각가의 섬세한 손길을 거쳐 평범한 돌멩이가 예술작품으로 태어난다",
            "신비로운 밤하늘의 별자리들을 바라보며 드넓은 우주를 꿈꿔본다",
            "용기란 두려움이 없는 것이 아니라 두려움에도 불구하고 나아가는 것이다",
            "타인을 용서하는 것은 나 자신을 얽매인 사슬에서 자유롭게 풀어주는 길이다",
        ]
    } else {
        &[
            "The quick brown fox jumps over the lazy dog.",
            "To be or not to be, that is the question.",
            "In the beginning God created the heaven and the earth.",
            "All that glitters is not gold, often have you heard that told.",
            "I think, therefore I am. Cogito, ergo sum.",
            "Ask not what your country can do for you, ask what you can do for your country.",
            "That's one small step for a man, one giant leap for mankind.",
            "Live as if you were to die tomorrow. Learn as if you were to live forever.",
            "Success is not final, failure is not fatal: it is the courage to continue that counts.",
            "The only way to do great work is to love what you do.",
            "In the middle of difficulty lies opportunity. Keep moving forward.",
            "Time is money, but it is also the most precious thing we can spend.",
            "Actions speak louder than words. Well begun is half done.",
            "Don't count the days, make the days count. Every moment is a fresh beginning.",
            "A friend in need is a friend indeed. Kindness is a language everyone understands.",
            "To love and be loved is to feel the sun from both sides.",
            "Believe you can and you're halfway there. Keep your dreams alive.",
            "Education is the most powerful weapon which you can use to change the world.",
            "Nothing is impossible, the word itself says I'm possible!",
            "Life is what happens when you're busy making other plans.",
            "The journey of a thousand miles begins with a single step.",
            "Good things come to those who wait, but better things to those who go and get them.",
            "It is during our darkest moments that we must focus to see the light.",
            "Do not go where the path may lead, go instead where there is no path and leave a trail.",
            "Many of life's failures are people who did not realize how close they were to success.",
            "You only live once, but if you do it right, once is enough.",
            "Be yourself; everyone else is already taken. Stay true to your heart.",
            "Two roads diverged in a wood, and I took the one less traveled by.",
            "In three words I can sum up everything I've learned about life: it goes on.",
            "The best and most beautiful things in the world cannot be seen or touched.",
            "Keep your face always toward the sunshine and shadows will fall behind you.",
            "Go confidently in the direction of your dreams. Live the life you've imagined.",
            "The only limit to our realization of tomorrow will be our doubts of today.",
            "If you want to live a happy life, tie it to a goal, not to people or things.",
            "Do not let making a living prevent you from making a life.",
            "Life is either a daring adventure or nothing at all.",
            "Strive not to be a success, but rather to be of value.",
            "You miss one hundred percent of the shots you don't take.",
            "The power of imagination makes us infinite and drives progress.",
            "Happiness is not something ready-made. It comes from your own actions.",
            "It always seems impossible until it is done. Keep striving.",
            "It does not matter how slowly you go as long as you do not stop.",
            "Our greatest glory is not in never falling, but in rising every time we fall.",
            "The supreme art of war is to subdue the enemy without fighting.",
            "The truth is rarely pure and never simple. Strive to find it.",
            "Try to be a rainbow in someone else's cloud. Bring them hope.",
            "We may encounter many defeats but we must not be defeated.",
            "You must be the change you wish to see in the world.",
            "What we think, we become. Cultivate positive thoughts.",
            "A warm smile is the universal language of kindness and love.",
            "A room without books is like a body without a soul. Read more.",
            "All generalizations are false, including this one. Think deeply.",
            "An unexamined life is not worth living. Explore your mind.",
            "Art is the lie that enables us to realize the truth.",
            "Be kind, for everyone you meet is fighting a harder battle.",
            "Beauty is in the eye of the beholder. Appreciate diversity.",
            "Change is the law of life. And those who look only to the past are certain to miss the future.",
            "Courage is grace under pressure. Stand tall against the storm.",
            "Creativity is intelligence having fun. Let your mind run free.",
            "Do what you can, with what you have, where you are.",
            "Don't cry because it's over, smile because it happened.",
            "Dream big and dare to fail. Great achievements require great risks.",
            "Everything you can imagine is real. The mind is a canvas.",
            "Genius is one percent inspiration and ninety-nine percent perspiration.",
            "He who has a why to live can bear almost any how.",
            "If you tell the truth, you don't have to remember anything.",
            "Innovation distinguishes between a leader and a follower.",
            "Integrity is doing the right thing, even when no one is watching.",
            "Knowledge speaks, but wisdom listens. Learn from everyone.",
            "Life is ten percent what happens to you and ninety percent how you respond.",
            "Logic will get you from A to B. Imagination will take you everywhere.",
            "Love all, trust a few, do wrong to none. Live in peace.",
            "Make each day your masterpiece. Paint your life with hope.",
            "No legacy is so rich as honesty. Be upright in all ways.",
            "Nothing is permanent in this wicked world, not even our troubles.",
            "One child, one teacher, one book, one pen can change the world.",
            "Patience is bitter, but its fruit is sweet. Endure and grow.",
            "Simplicity is the ultimate sophistication. Keep it clean and clear.",
            "The best way to predict your future is to create it.",
            "The only true wisdom is in knowing you know nothing.",
            "There is no path to peace. Peace is the path. Walk with love.",
            "Think of all the beauty still left around you and be happy.",
            "Those who cannot change their minds cannot change anything.",
            "To love oneself is the beginning of a lifelong romance.",
            "Well done is better than well said. Let actions show.",
            "What lies behind us and what lies before us are tiny matters compared to what lies within us.",
            "Whatever you are, be a good one. Strive for excellence.",
            "Whenever you find yourself on the side of the majority, it is time to pause and reflect.",
            "With the new day comes new strength and new thoughts.",
            "Yesterday is history, tomorrow is a mystery, today is a gift.",
        ]
    }
}

pub fn long_text_body(is_korean: bool, text_idx: usize) -> &'static str {
    if is_korean {
        match text_idx {
            0 => "\
동해 물과 백두산이 마르고 닳도록 하느님이 보우하사 우리나라 만세.
남산 위에 저 소나무 철갑을 두른 듯 바람 서리 불변함은 우리 기상일세.
가을 하늘 공활한데 높고 구름 없이 밝은 달은 우리 가슴 일편단심일세.
이 기상과 이 맘으로 충성을 다하여 괴로우나 즐거우나 나라 사랑하세.
무궁화 삼천리 화려 강산 대한 사람 대한으로 길이 보전하세.",
            1 => "\
나랏말싸미 듕귁에 달아 문자와로 서르 사맛디 아니할쎄
이런 젼차로 어린 백셩이 니르고져 홇배이셔도
마참내 제 뜨들 시러 펴디 못할 노미 하니라
내 이랄 위하야 어여삐 너겨 새로 스물여덟 자랄 맹가노니
사람마다 해여 수비 니겨 날로 쑤메 편안케 하고져 할 따름이니라",
            2 => "\
나 보기가 역겨워 가실 때에는 말없이 고이 보내 드리우리다.
영변에 약산 진달래꽃 아름 따다 가실 길에 뿌리우리다.
가시는 걸음 걸음 놓인 그 꽃을 사뿐히 즈려밟고 가시옵소서.
나 보기가 역겨워 가실 때에는 죽어도 아니 눈물 흘리우리다.",
            _ => "\
대한민국은 민주공화국이다. 대한민국의 주권은 국민에게 있고, 모든 권력은 국민으로부터 나온다.
대한민국 국민이 되는 요건은 법률로 정한다. 국가는 재외국민을 보호할 의무를 진다.
대한민국의 영토는 한반도와 그 부속도서로 한다. 대한민국은 통일을 지향하며 평화적 통일 정책을 수립하고 추진한다.
대한민국은 국제평화의 유지에 노력하고 침략적 전쟁을 부인한다. 국군은 국가의 안전보장과 국토방위의 의무를 수행한다.",
        }
    } else {
        match text_idx {
            0 => "\
Four score and seven years ago our fathers brought forth on this continent, a new nation, conceived in Liberty, and dedicated to the proposition that all men are created equal.
Now we are engaged in a great civil war, testing whether that nation, or any nation so conceived and so dedicated, can long endure.
We are met on a great battle-field of that war. We have come to dedicate a portion of that field, as a final resting place for those who here gave their lives that that nation might live.",
            1 => "\
I say to you today, my friends, so even though we face the difficulties of today and tomorrow, I still have a dream.
It is a dream deeply rooted in the American dream.
I have a dream that one day this nation will rise up and live out the true meaning of its creed: We hold these truths to be self-evident, that all men are created equal.
I have a dream that my four little children will one day live in a nation where they will not be judged by the color of their skin but by the content of their character.",
            _ => "\
Two roads diverged in a yellow wood, and sorry I could not travel both and be one traveler, long I stood.
And looked down one as far as I could to where it bent in the undergrowth; then took the other, as just as fair.
And having perhaps the better claim, because it was grassy and wanted wear; though as for that the passing there had worn them really about the same.
I shall be telling this with a sigh somewhere ages and ages hence: Two roads diverged in a wood, and I took the one less traveled by, and that has made all the difference.",
        }
    }
}
