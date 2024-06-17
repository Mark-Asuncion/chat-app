import { lineWobble } from 'ldrs'

export default function Loading() {
    lineWobble.register()
    return (
        <div className='fixed top-1/2 left-1/2 translate-x-[-50%] translate-y-[-50%]'>
            <h1 className='text-accent text-center text-xl'>Loading</h1>
            <l-line-wobble
                size="150"
                stroke="2"
                bg-opacity="0.1"
                speed="1.75"
                color="#a667f3"
            ></l-line-wobble>
        </div>
    );
}
