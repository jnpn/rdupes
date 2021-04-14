use std::thread;
use walkdir::WalkDir;
use std::channels;

mod mt {

    /*
     *
     * scan-thread -> <scan-queue> -> hash-thread -> <hash-q> -> map-thread -> <map-q> -> screen-thread
     *
     * - scan-p: (fn, ctime)
     * - hash-p: (fn, ctime, hash)
     * - map-p : (hash, [(fn, ctime)]), hash-count
     *
     * - screen-s: { scanned-count hash-count start-time delta-time }
     *
     */

    fn twalk(dir, tx, f) {
	thread::spawn(||{
	    for fn in walkdir(dir) {
		tx.send(f(fn))
	    }
	})
    }

    fn thash(rx, tx) {}

    fn tmap(rx, tx) {}

    fn tshow(rx) {
	let s = Screen {
	    scanned: 0,
	    hashed: 0,
	    start: 0,
	    delta: 0
	}
	for m in rx {
	    // update s
	}
    }

}
