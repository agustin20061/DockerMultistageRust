# DockerMultistageRust
Fernandez Cristian, Gliksman Agustin

Para poder buildear la imagen del dockerfile MultiStage es necesario poner este comando en una terminal ubicada en la carpeta helloworld :
    
    sudo docker build -t multistage -f dockerfileMulti .
Para poder correr esta imagen se debe correr este comando:
    
    sudo docker run -p 3030:3030 multistage


Para poder buildear la imagen del dockerfile Singlestage es necesario poner este comando en una terminal ubicada en la carpeta helloworld :
    
    sudo docker build -t singlestage -f dockerfileSingle .
Para poder correr esta imagen se debe correr este comando:
    
    sudo docker run -p 3030:3030 singlestage

