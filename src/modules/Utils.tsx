import React, { ReactNode } from 'react';

export interface TabEntry {
    active?: boolean,
    text: string,
    cb: () => void
}

export function Tabs({ entries }: { entries: TabEntry[] }) {
    let nentries: ReactNode[] = [];
    entries.forEach((v,i) => {
        const active = (v.active === true)? "border-accent text-accent":"";
        let classes = 'hover:border-accent p-2 font-medium border-b mb-6' + ` ${active}`;
        nentries.push((
            <button key={i} type='button' className={classes} onClick={() => {
                v.cb();
            }}>
                {v.text}
            </button>
        ));
    });
    return (
        <div className='mx-auto flex flex-row justify-center w-1/2'>
            {nentries}
        </div>
    )
}
