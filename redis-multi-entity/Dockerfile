FROM jupyter/base-notebook:latest

# Install Redis server
USER root
RUN apt-get update && apt-get install -y redis-server
RUN echo "bind 0.0.0.0" >> /etc/redis/redis.conf

COPY start.sh ./
RUN chmod +x ./start.sh

# Install Jupyter Lab extensions
USER jovyan
# RUN conda install --quiet --yes redis
RUN mkdir ntbks
RUN conda install --quiet --yes -c anaconda redis

# Expose the Redis port and start Jupyter Lab
EXPOSE 8888 6379
CMD ["./start.sh"]
