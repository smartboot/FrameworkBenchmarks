FROM maven:3.9.9-eclipse-temurin-21 as maven
WORKDIR /inverno
COPY src src
COPY pom.xml pom.xml
RUN mvn package -q -Pio.inverno.epoll

EXPOSE 8080

CMD export DBIP=`getent hosts tfb-database | awk '{ print $1 }'` && \
    target/inverno-benchmark-1.0.0-SNAPSHOT-application_linux_amd64/bin/inverno-benchmark \
    --com.techempower.inverno.benchmark.appConfiguration.db_host=\"$DBIP\" \
    --com.techempower.inverno.benchmark.appConfiguration.boot.reactor_event_loop_group_size=$((`grep --count ^processor /proc/cpuinfo`))
