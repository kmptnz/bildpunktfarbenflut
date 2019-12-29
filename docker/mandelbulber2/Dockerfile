FROM nvidia/opencl:latest

# Build-time metadata as defined at http://label-schema.org

LABEL org.label-schema.build-date=$BUILD_DATE \
      org.label-schema.name="mandelbulber2 docker" \
      org.label-schema.description="A quick pixelflut (video) client in Rust for use at 34C3, that pwns whole pixelflut panels" \
      org.label-schema.url="https://gitlab.com/kmptnz" \
      org.label-schema.vendor="kmpt.nz" \
      org.label-schema.version=$CI_COMMIT_REF_SLUG \
      org.label-schema.schema-version="1.0"

RUN apt update -q && \
    apt install -y --no-install-recommends build-essential libqt5gui5 qt5-default libpng16-16 libpng-dev qttools5-dev qttools5-dev-tools \
	libgomp1 libgsl-dev libsndfile1-dev qtmultimedia5-dev libqt5multimedia5-plugins liblzo2-2 liblzo2-dev qtcreator git ca-certificates \
    clinfo opencl-headers ocl-icd-libopencl1

RUN git clone https://github.com/buddhi1980/mandelbulber2

WORKDIR /mandelbulber2/mandelbulber2

ENV export CXXFLAGS="-march=native -msse2"
ENV MANDELBULBER_SHARE="/usr/share/mandelbulber2"

RUN ls -lah && cd qmake && qmake mandelbulber-opencl.pro && make -j12

RUN install qmake/mandelbulber2 /usr/bin

RUN mkdir $MANDELBULBER_SHARE && \
ln -s ${PWD}/formula $MANDELBULBER_SHARE/formula && \
ln -s ${PWD}/deploy/share/mandelbulber2/data $MANDELBULBER_SHARE/data && \
ln -s ${PWD}/language $MANDELBULBER_SHARE/language && \
ln -s ${PWD}/deploy/share/mandelbulber2/materials $MANDELBULBER_SHARE/materials && \
ln -s ${PWD}/deploy/share/mandelbulber2/examples $MANDELBULBER_SHARE/examples && \
ln -s ${PWD}/deploy/share/mandelbulber2/icons $MANDELBULBER_SHARE/icons && \
ln -s ${PWD}/deploy/share/mandelbulber2/textures $MANDELBULBER_SHARE/textures && \
ln -s ${PWD}/deploy/share/mandelbulber2/toolbar $MANDELBULBER_SHARE/toolbar && \
ln -s ${PWD}/deploy/doc $MANDELBULBER_SHARE/doc && \
ln -s ${PWD}/deploy/share/mandelbulber2/sounds $MANDELBULBER_SHARE/sounds && \
ln -s ${PWD}/opencl $MANDELBULBER_SHARE/opencl

ENTRYPOINT ["/usr/bin/mandelbulber2", "-n"]