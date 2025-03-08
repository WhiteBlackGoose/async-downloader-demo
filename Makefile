run-server:
	@cd _serv && filebrowser --port 8000

run-python:
	@cd python && pyright
	@cd python && time python src/main.py 2> /dev/null

run-dotnet:
	@cd dotnet && dotnet publish -r linux-x64 -c Release
	@time dotnet/bin/Release/net9.0/linux-x64/publish/demo 2> /dev/null

run-rust:
	@cd rust && cargo build --release
	@time rust/target/release/demo 2> /dev/null
