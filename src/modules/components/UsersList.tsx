import React from 'react';

export interface UserEntry {
    selected?:     boolean,
    profileImg:    string,
    name:          string,
    message:       string,
    // in seconds
    lastSent:      number,
    onClick?:      (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => void
}

export interface UsersListProps {
    // children?:     JSX.Element | JSX.Element[],
    className?:    string,
    entries:       UserEntry[]
}

export default function UsersList(props: UsersListProps) {
    const classN = ( props.className )? ` ${props.className}`:"";
    return (
        <div className={ "flex flex-col" + classN }>
            {
                props.entries.map((v, i) => {
                    return (
                        <User key={i}
                            profileImg={v.profileImg}
                            name={v.name}
                            message={v.message}
                            lastSent={v.lastSent}
                            onClick={v.onClick}
                            selected={v.selected}
                        />
                    )
                })
            }
        </div>
    )
}

export function secToTimeStr(sec: number): string {
    // TODO use Date class
    if (sec < 60) {
        return sec + "s";
    }
    const min = Math.floor( sec / 60 );
    if (min >= 60) {
        // get also day and year
        return `${Math.floor( min / 60)}hr`;
    }
    return `${min}min`;
}

function User(props: UserEntry) {
    // truncate if more than ???
    const message = props.message;
    const lastSent = secToTimeStr(props.lastSent);
    const selected = (props.selected == true)? " bg-neutral-700":" hover:bg-neutral-800";
    return (
        <div className={ 'relative w-full rounded-md'
            + ' text-white'
            + ' flex flex-row items-center p-2' + selected }
            onClick={(e) => {
                if (props.onClick)
                    props.onClick(e);
            }}>
            <div className=' bg-white rounded-full h-full aspect-square overflow-hidden
                mr-2'>
                <img src={props.profileImg} alt={props.profileImg}
                    className='bg-cover'/>
            </div>
            <div>
                <h1>{props.name}</h1>
                <span className='text-sm'>{message}</span>
            </div>
            <span className='text-xs absolute inset-y-0 right-0 h-max
                pr-3 pb-3 mt-auto'>{lastSent}</span>
        </div>
    )
}
