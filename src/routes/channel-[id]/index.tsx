import { component$ } from '@builder.io/qwik'
import { useLocation } from '@builder.io/qwik-city'
import { ChannelBanner } from '~/components/channel-banner/channel-banner'
import { Thumbnail } from '~/components/thumbnail/thumbnail'

export default component$(() => {
    const loc = useLocation()
    const videos: string[] = []

    return (
        <>
            <ChannelBanner channelId={loc.params.id} />
            <div>
                {videos.map((v) => (
                    <Thumbnail
                        key={`thumbnail-${v}`}
                        videoId={v}
                        style="channelPage"
                    />
                ))}
            </div>
        </>
    )
})
