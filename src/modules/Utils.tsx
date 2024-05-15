import React, { ReactNode, useEffect, useState } from 'react';

export interface TabEntry {
    active?: boolean,
    text: string,
    cb: () => void
}

export function Tabs({ entries }: { entries: TabEntry[] }) {
    const [ tabEntries, setTabEntries ] = useState(entries);
    function changeActive(index: number) {
        setTabEntries((prev) => {
            return prev.map((v, i) => {
                let active = false;
                if (i === index) {
                    active = true;
                }
                return {
                    ...v,
                    active
                };
            });
        });
    }

    let nentries: ReactNode[] = [];
    tabEntries.forEach((v,i) => {
        const active = (v.active === true)? "border-accent text-accent":"";
        let classes = 'hover:border-accent p-2 font-medium border-b mb-6' + ` ${active}`;
        nentries.push((
            <button key={i} type='button' className={classes} onClick={() => {
                changeActive(i);
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
