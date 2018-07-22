while true; do
	clear;
	cargo run;
	inotifywait -e CLOSE_WRITE `git ls-files .`;
done
