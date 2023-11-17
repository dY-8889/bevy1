import glob
import os

from pathlib import Path
from PIL import Image, ImageSequence

# 分割したいアニメーション GIF 画像
IMAGE_PATH = glob.glob("assets/images/*.gif")


def get_frames(path):
    """パスで指定されたファイルのフレーム一覧を取得する"""
    im = Image.open(path)
    return (frame.copy() for frame in ImageSequence.Iterator(im))


def write_frames(frames, name_original):
    """フレームを別個の画像ファイルとして保存する"""
    path = Path(name_original)

    # ファイル名
    stem = path.stem

    # 出力先のディレクトリが存在しなければ作成
    dir = "assets/images/" + stem

    print(dir)
    if not os.path.exists(dir):  # ディレクトリが存在するか確認
        os.makedirs(dir)  # ディレクトリ作成

        print('Destionation directory is created: "{}".'.format(stem))

        for i, f in enumerate(frames):
            name = "assets/images/{}/{}{}".format(stem, i + 1, ".png")
            f.save(name)

            print('A frame is saved as "{}".'.format(name))


if __name__ == "__main__":
    for img_path in IMAGE_PATH:
        frames = get_frames(img_path)
        write_frames(frames, img_path)

        os.remove(img_path)
