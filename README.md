# avesync

Hálózati fájlrendszer szinkronizáló eszköz.

## Felépítés

Kettő vagy több daemon kapcsolódik egymáshoz és egy központi adatbázishoz. Minden kliens a saját helyi fájlrendszerében lévő fájljainak neveit és meta adatait a központi adatbázisba tölti fel. Időközönént a kliens összehasonlítja a saját fájlistáját egy közpönti adatbázisban lévő fájllistával és ha hiányoznak fájlok akkor letölti egy másik klienstől, ha pedig az adatbázisban már nem létezik, de a kliensnél igen, akkor letörli azt.

## Adatbázis struktúra

- file
  - filepath (relatív útvonal)
  - size
  - file_date (fájlrendszerben tárolt időpont (Change))
  - replicas (egy lista a kliensekről, akiknél létezik a fájl, pl.: 'szeged-1,budapest-4')
  - to_delete (bool, ha true, akkor a fájlt törölni kell minden replikán)
  - updated_at (az adott db sor utolsó módosításának ideje)
 
- replicas
  - name
  - ip
  - port
  - connected_at
  - is_online (amikor indul a binaris akkor true, amikor lekapcsolódik akkor false)
    
## Indítás
A kliens induláshoz szükség van:
- kliens névre (ez kerül majd be replicas-ba)
- kliens ip címe és portja (bekerül a db-be hogy a többiek tudják)
- egy mappa útvonalra (ez lesz szinkronizálva)
- egy adatbázis host-ra

## Működés
**Létrehozás:**
- Fájlrendszer szkennelés közben kiderül, hogy a db-ben nem létezik, de a fájlrendszerben igen, ilyenkor létrehozza a fájl sort db-ben és beírja magát a replicas-ba.
- A db szkennelés közben kiderül, hogy db-ben létezik a fájl, de a fájlrendszerben nem, akkor letölti valamelyik replicas-tól és beírja magát a replicas-ok közé.
  
**Módosítás:**
- Ha szkennelés közben kiderül, hogy a saját fájlrendszerben lévő fájl dátuma újabb mint a db-ben lévő, akkor a replicas-ból kitöröl mindenki mást, csak magát hagyja meg (ezért a többiek lefogja újra tölteni a fájlt).

**Törlés:**
- Ha szkennelés közben nincs meg a fájl, de a db-ben az van, hogy nála van a fájl (benne van a kliens a replicas-ba), akkor to_delete = true és kiveszi magát a replikából (ha ő volt az utolsó replika akkor a sort is törli)

- Ha db-ben talál egy olyan fájlt aminek a to_delete értéke true, nála létezik a fájl és a replicas-ba is szerepel, akkor törli a fájlt és kiveszi magát a replicasból (ha ő volt az utolsó replika akkor a sort is törli)

## TODO:
Konfliktusok, pl. mindkét kliensen ugyanazzal a fájlnévvel jön létre, akkor melyik az erősebb. Az újabb? Tartsuk meg mindkettőt _conflict suffix-el?
