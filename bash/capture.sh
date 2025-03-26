ffmpeg -f avfoundation -r 25 -i 1 -filter_complex "[0:v]split=2[out1][out2]"   \
    -map "[out1]" -c:v libx264 -f hls -hls_time 2 -hls_list_size 0 -hls_flags delete_segments -hls_segment_filename "segment_%03d.ts" playlist.m3u8 \
    -map "[out2]" -f rawvideo -
