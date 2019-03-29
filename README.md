[![crates.io](https://img.shields.io/crates/v/locationsharing.svg)](https://crates.io/crates/locationsharing)
[![docs.rs](https://docs.rs/locationsharing/badge.svg)](https://docs.rs/locationsharing/latest/locationsharing/)
[![Build Status](https://travis-ci.org/stillinbeta/locationsharing.svg?branch=master)](https://travis-ci.org/stillinbeta/locationsharing) 

# LocationSharing

Accesses a certain colourful company's undocumented API for location sharing.
Cribbed from [coastf's Python implementation](https://github.com/costastf/locationsharinglib).

## Cookies

This API is not like other APIs, in that Oauth credentials are not sufficient for access.
The only way to access it is with regular session credentials. 

The easiest way to obtain these is logging into your account on a web browser and copying the cookies out.
