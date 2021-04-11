EAPI="7"

inherit eutils cargo git-r3

DESCRIPTION="plain-text command-line task management utility"
HOMEPAGE="https://github.com/paradigm/${PN}"
EGIT_REPO_URI="https://github.com/paradigm/${PN}"

LICENSE="MIT"
SLOT="0"
KEYWORDS="amd64"
IUSE=""

DEPEND=">=virtual/rust-1.52.0"

src_unpack() {
	default
	git-r3_src_unpack
	cargo_live_src_unpack
}

src_compile() {
	cargo_gen_config
	default
}

src_install() {
	cargo_src_install

	insinto /usr/share/zsh/site-functions/
	doins "${WORKDIR}/${P}/third-party-tooling-support/zsh/_chore"

	insinto /usr/share/vim/ftdetect/
	doins "${WORKDIR}/${P}/third-party-tooling-support/vim/ftdetect/chore.vim"

	insinto /usr/share/vim/syntax/
	doins "${WORKDIR}/${P}/third-party-tooling-support/vim/syntax/chore.vim"
}
