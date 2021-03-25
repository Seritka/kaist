# kaist
KAIST API library

### example
```rust
use kaist::{request, RequestOpts};

fn main() {
    let opts = RequestOpts::new("뉴스", "국문", None, None);
    let data = request(opts).unwrap();

    for i in data {
        println!("게시물 출력 일년번호: {}\n게시물 관리 번호: {}\n게시판 코드: {}\n게시판명: {}\n게시판 글 제목: {}\n게시판 내용: {}\n게시글 등록일: {}\n이미지 URL (뉴스만 제공): {}\n게시글 상세보기 링크 주소: {}\n\n", i.no(), i.ntt_no(), i.bbs_cd(), i.bbs_nm(), i.subject(), i.contents(), i.reg_date(), i.img(), i.view_link());
    }
}
```