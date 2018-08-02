while true; do
	clear;
	cargo clean -p raytracer;
	if [ -z "$1" ]
	then
		time cargo run;
	else
		time cargo $@;
	fi
	inotifywait -e CLOSE_WRITE `git ls-files .`;
done
