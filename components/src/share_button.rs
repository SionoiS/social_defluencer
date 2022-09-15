#![cfg(target_arch = "wasm32")]

use cid::Cid;

use crate::thumbnail::Thumbnail;

use defluencer::channel::{local::LocalUpdater, Channel};

use gloo_console::error;

use utils::defluencer::ChannelContext;

use ybc::{Box, Button};

use yew::{platform::spawn_local, prelude::*};

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Signed link to media Cid
    pub cid: Cid,
}

pub struct ShareButton {
    channel: Option<Channel<LocalUpdater>>,

    share_cb: Callback<MouseEvent>,

    modal_cb: Callback<MouseEvent>,
    modal: bool,
}

pub enum Msg {
    Modal,
    Share,
}

impl Component for ShareButton {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let channel = ctx
            .link()
            .context::<ChannelContext>(Callback::noop())
            .map(|(context, _)| context.channel);

        let share_cb = ctx.link().callback(|_| Msg::Share);
        let modal_cb = ctx.link().callback(|_| Msg::Modal);

        Self {
            channel,

            share_cb,

            modal_cb,
            modal: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Modal => self.on_click(),
            Msg::Share => self.on_share(ctx),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.channel.is_none() {
            return html! {
            <Button disabled={true} >
                <span class="icon">
                    <i class="fa-solid fa-reply"></i>
                </span>
            </Button>
            };
        }

        html! {
        <>
            <Button classes={classes!("is-outlined")} onclick={self.modal_cb.clone()} >
                <span class="icon">
                    <i class="fa-solid fa-reply"></i>
                </span>
            </Button>
            { self.render_modal(ctx) }
        </>
        }
    }
}
impl ShareButton {
    fn render_modal(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class= { if self.modal { "modal is-active" } else { "modal" } } >
            <div class="modal-background" onclick={self.modal_cb.clone()} ></div>
            <div class="modal-content">
                <Box>
                    <Thumbnail cid={ctx.props().cid} />
                </Box>
                <Box>
                    <Button onclick={self.share_cb.clone()} >
                        { "Share" }
                    </Button>
                </Box>
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={self.modal_cb.clone()} />
        </div>
        }
    }

    fn on_click(&mut self) -> bool {
        self.modal = !self.modal;

        true
    }

    fn on_share(&mut self, ctx: &Context<Self>) -> bool {
        if self.channel.is_none() {
            return false;
        }

        let channel = self.channel.as_ref().unwrap().clone();

        spawn_local(share_content(channel, ctx.props().cid));

        self.modal = false;

        true
    }
}

async fn share_content(channel: Channel<LocalUpdater>, cid: Cid) {
    if let Err(e) = channel.add_content(cid).await {
        error!(&format!("{:#?}", e))
    }
}
