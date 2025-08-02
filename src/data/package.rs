pub struct Package<'a> {
    tld: &'a str,
    domain: &'a str,
    application: &'a str,
}

impl Package<'_> {
    pub fn tld(&self) -> &str {
        self.tld
    }

    pub fn domain(&self) -> &str {
        self.domain
    }

    pub fn application(&self) -> &str {
        self.application
    }
}

const PACKAGE: Package = Package {
    tld: "com",
    domain: "Aria AzadiPour",
    application: "Arfima",
};

pub fn get() -> &'static Package<'static> {
    &PACKAGE
}
