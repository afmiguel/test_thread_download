import os
import math
from PIL import Image

# Limite máximo desejado (abaixo dos 25MB do GitHub)
MAX_SIZE_MB = 24.5  
MAX_SIZE_BYTES = MAX_SIZE_MB * 1024 * 1024

def upscale_dynamic_to_target():
    for i in range(10):
        filename = f"arquivo_{i}.jpg"
        out_filename = f"large_{filename}"
        
        if not os.path.exists(filename):
            print(f"Arquivo {filename} não encontrado. Pulando...")
            continue
            
        with Image.open(filename) as img:
            original_size = os.path.getsize(filename)
            if original_size == 0:
                continue
                
            # Calcula quantas vezes o arquivo precisa crescer em bytes
            size_multiplier = MAX_SIZE_BYTES / original_size
            
            # A escala linear (largura/altura) é a raiz quadrada do multiplicador de área.
            # Multiplicamos por 1.2 para garantir um pequeno excesso inicial e forçar o ajuste fino de qualidade.
            dynamic_scale = math.sqrt(size_multiplier) * 1.2 
            
            new_width = int(img.width * dynamic_scale)
            new_height = int(img.height * dynamic_scale)
            
            # Trava de segurança para evitar estouro de memória RAM em imagens minúsculas
            if new_width > 22000 or new_height > 22000:
                scale_limit = 22000 / max(new_width, new_height)
                new_width = int(new_width * scale_limit)
                new_height = int(new_height * scale_limit)
            
            print(f"[{filename}] Original: {(original_size/1024/1024):.2f} MB -> Alvo: ~24.5 MB")
            print(f"  Redimensionando: {img.width}x{img.height} -> {new_width}x{new_height} (Escala linear: {dynamic_scale:.1f}x)")
            
            # Resampling de alta qualidade
            resized_img = img.resize((new_width, new_height), Image.Resampling.LANCZOS)
            
            # Ajuste dinâmico de qualidade para encaixar o arquivo no limite
            quality = 100
            while quality > 5:
                resized_img.save(out_filename, "JPEG", quality=quality)
                current_size = os.path.getsize(out_filename)
                
                if current_size <= MAX_SIZE_BYTES:
                    break
                
                quality -= 2 
            
            final_size_mb = current_size / (1024 * 1024)
            print(f"  Salvo: {out_filename}")
            print(f"  Peso final alcançado: {final_size_mb:.2f} MB (Qualidade: {quality})\n")

if __name__ == "__main__":
    # Aumenta o limite de pixels permitidos pelo Pillow para evitar erro em resoluções gigantes
    Image.MAX_IMAGE_PIXELS = None 
    upscale_dynamic_to_target()