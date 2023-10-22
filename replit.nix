{ pkgs }: {
	deps = with pkgs; [
        cargo
        rustc
        rustfmt
        rust-analyzer
        mold
    ];
}
