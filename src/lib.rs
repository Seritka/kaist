use serde::{Deserialize, Serialize};

/// code: (알림사항: 0801, 학사공지: 0802, 채용초빙: kr_0814, 뉴스: news)
/// site_dvs_cd: (국문: kr, 영문: en)
/// display: (10~100)
/// start: (1~1000)
pub struct RequestOpts {
    code: String,
    site_dvs_cd: String,
    display: Option<i32>,
    start: Option<i32>,
}

impl RequestOpts {
    pub fn new(code: &str, site_dvs_cd: &str, display: Option<i32>, start: Option<i32>) -> Self {
        RequestOpts {
            code: code.to_string(),
            site_dvs_cd: site_dvs_cd.to_string(),
            display,
            start,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Respone {
    no: String,
    ntt_no: String,
    bbs_cd: String,
    bbs_nm: String,
    subject: String,
    contents: String,
    reg_date: String, //NOTE: datetime
    img: String,
    view_link: String,
}

impl Respone {
    pub fn no(&self) -> usize {
        self.no.parse::<usize>().unwrap()
    }
    pub fn ntt_no(&self) -> usize {
        self.ntt_no.parse::<usize>().unwrap()
    }
    pub fn bbs_cd(&self) -> String {
        self.bbs_cd.clone()
    }
    pub fn bbs_nm(&self) -> String {
        self.bbs_nm.clone()
    }
    pub fn subject(&self) -> String {
        self.subject.clone()
    }
    pub fn contents(&self) -> String {
        self.contents.clone()
    }
    pub fn reg_date(&self) -> String {
        self.reg_date.clone()
    }
    pub fn img(&self) -> String {
        self.img.clone()
    }
    pub fn view_link(&self) -> String {
        self.view_link.clone()
    }
}

///
///```rust
/// use kaist::{request, RequestOpts};
///
/// fn main() {
///    let opts = RequestOpts::new("뉴스", "국문", None, None);
///    let data = request(opts).unwrap();
///
///    for i in data {
///        println!("게시물 출력 일년번호: {}\n게시물 관리 번호: {}\n게시판 코드: {}\n게시판명: {}\n게시판 글 제목: {}\n게시판 내용: {}\n게시글 등록일: {}\n이미지 URL (뉴스만 제공): {}\n게시글 상세보기 링크 주소: {}\n\n", i.no(), i.ntt_no(), i.bbs_cd(), i.bbs_nm(), i.subject(), i.contents(), i.reg_date(), i.img(), i.view_link());
///    }
/// }
///
pub fn request(opts: RequestOpts) -> Result<Vec<Respone>, String> {
    let code = if opts.code == "알림사항".to_string() {
        "0801".to_string()
    } else if opts.code == "학사공지".to_string() {
        "0802".to_string()
    } else if opts.code == "채용초빙".to_string() {
        "kr_0814".to_string()
    } else if opts.code == "뉴스".to_string() {
        "news".to_string()
    } else if opts.code == "0801".to_string()
        || opts.code == "0802".to_string()
        || opts.code == "kr_0814".to_string()
        || opts.code == "news".to_string()
    {
        opts.code
    } else {
        return Err("지원 되지 않은 코드입니다.".to_string());
    };

    let site = if opts.site_dvs_cd == "국문" {
        "kr".to_string()
    } else if opts.site_dvs_cd == "영문" {
        "en".to_string()
    } else if opts.site_dvs_cd == "kr" || opts.site_dvs_cd == "en" {
        opts.site_dvs_cd
    } else {
        return Err("지원 되지 않은 사이트코드입니다.".to_string());
    };

    let display = if opts.display.is_none() {
        10
    } else if 9 < opts.display.unwrap() && opts.display.unwrap() < 101 {
        opts.display.unwrap()
    } else {
        return Err("최소 10부터 최대 100까지 가능합니다.".to_string());
    };

    let start = if opts.start.is_none() {
        1
    } else if 0 < opts.start.unwrap() && opts.start.unwrap() < 1001 {
        opts.start.unwrap()
    } else {
        return Err("최소 1부터 최대 1000까지 가능합니다.".to_string());
    };

    let url = format!(
        "http://www.kaist.ac.kr/_module/api/json.php?code={}&site_dvs_cd={}&display={}&start={}",
        code, site, display, start
    );

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            format!("AkiaCode/kaist v1.0 (https://github.com/AkiaCode)"),
        )
        .send()
        .unwrap()
        .text()
        .unwrap();

    let json: Vec<Respone> = serde_json::from_str(&res).unwrap();

    Ok(json)
}
