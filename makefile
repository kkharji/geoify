# TODO: Simplify setup and get rid of conda
# Manually run: conda activate geoify

nlp-conda-init:
	conda create -n geoify python=3.8

nlp-setup-torch:
	conda install pytorch
	ln -s /opt/homebrew/Caskroom/miniforge/base/envs/geoify/lib/python3.8/site-packages/torch/ torch
	mkdir -p ~/lib; ln -sf ./torch/lib/* ~/lib
