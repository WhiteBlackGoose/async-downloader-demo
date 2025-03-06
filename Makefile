run-server:
	@cd _serv && python -m http.server --bind 127.0.0.1

run-python:
	@cd python && pyright
	@cd python && time python src/main.py 2> /dev/null
