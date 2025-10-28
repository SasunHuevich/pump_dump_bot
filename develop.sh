docker build -t develop_container -f Dockerfile.develop .

docker run -it --rm --name develop_container -v $(pwd):/pump_dump_bot -p 8080:80 develop_container bash