while true; do
	clear;
	cargo clean -p raytracer;
	time cargo run;
	inotifywait -e CLOSE_WRITE `git ls-files .`;
done
