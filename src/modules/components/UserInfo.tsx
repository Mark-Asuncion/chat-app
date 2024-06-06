import React from 'react';

export interface IUserInfo {
    id:         string,
    username:   string,
    email:      string
}

export interface UserInfoProps {
    className?:    string,
    profileImg:    string,
    name:          string,
    onClick?:      (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => void
}

export default function UserInfo(props: UserInfoProps) {
    const classN = ( props.className )? ` ${props.className}`:"";
    return (
        <div className={'relative w-full rounded-md text-white'
            + ' flex flex-row items-center p-2'
            + ' hover:bg-neutral-800' + classN}
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
            </div>
        </div>
    )
}
