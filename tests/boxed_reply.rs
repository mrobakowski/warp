#![deny(warnings)]
extern crate pretty_env_logger;
extern crate warp;

use warp::Filter;

#[test]
fn boxed_str_reply() {
    let boxed = warp::any().map(|| warp::reply::boxed("foo"));

    let req = warp::test::request();
    let resp = req.reply(&boxed);

    assert_eq!(resp.status(), 200);
    assert_eq!(&resp.body()[..], b"foo");
}

#[test]
fn boxed_reply_use_case() {
    let boxed_alternative = warp::path::param2()
        .map(|b: bool| if b { warp::reply::boxed("foo") } else { warp::reply::boxed(warp::reply()) });

    let req = warp::test::request().path("/true");
    let resp = req.reply(&boxed_alternative);

    assert_eq!(resp.status(), 200);
    assert_eq!(&resp.body()[..], b"foo");

    let req = warp::test::request().path("/false");
    let resp = req.reply(&boxed_alternative);

    assert_eq!(resp.status(), 200);
    assert!(resp.body().is_empty());
}

#[test]
fn boxed_complex_reply() {
    let header = warp::reply::with::header("foo", "bar");

    let no_header = warp::any().map(|| "baz").with(&header).map(warp::reply::boxed);

    let req = warp::test::request();
    let resp = req.reply(&no_header);
    assert_eq!(resp.headers()["foo"], "bar");
    assert_eq!(&resp.body()[..], b"baz");
}