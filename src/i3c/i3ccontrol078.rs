#[doc = "Register `I3CCONTROL078` reader"]
pub type R = crate::R<I3ccontrol078Spec>;
#[doc = "Register `I3CCONTROL078` writer"]
pub type W = crate::W<I3ccontrol078Spec>;
#[doc = "Field `REGAUTOCMDDEV112` reader - REG_AUTOCMD_DEV_112"]
pub type Regautocmddev112R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV112` writer - REG_AUTOCMD_DEV_112"]
pub type Regautocmddev112W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV113` reader - REG_AUTOCMD_DEV_113"]
pub type Regautocmddev113R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV113` writer - REG_AUTOCMD_DEV_113"]
pub type Regautocmddev113W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV114` reader - REG_AUTOCMD_DEV_114"]
pub type Regautocmddev114R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV114` writer - REG_AUTOCMD_DEV_114"]
pub type Regautocmddev114W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV115` reader - REG_AUTOCMD_DEV_115"]
pub type Regautocmddev115R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV115` writer - REG_AUTOCMD_DEV_115"]
pub type Regautocmddev115W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV116` reader - REG_AUTOCMD_DEV_116"]
pub type Regautocmddev116R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV116` writer - REG_AUTOCMD_DEV_116"]
pub type Regautocmddev116W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV117` reader - REG_AUTOCMD_DEV_117"]
pub type Regautocmddev117R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV117` writer - REG_AUTOCMD_DEV_117"]
pub type Regautocmddev117W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV118` reader - REG_AUTOCMD_DEV_118"]
pub type Regautocmddev118R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV118` writer - REG_AUTOCMD_DEV_118"]
pub type Regautocmddev118W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV119` reader - REG_AUTOCMD_DEV_119"]
pub type Regautocmddev119R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV119` writer - REG_AUTOCMD_DEV_119"]
pub type Regautocmddev119W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_112"]
    #[inline(always)]
    pub fn regautocmddev112(&self) -> Regautocmddev112R {
        Regautocmddev112R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_113"]
    #[inline(always)]
    pub fn regautocmddev113(&self) -> Regautocmddev113R {
        Regautocmddev113R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_114"]
    #[inline(always)]
    pub fn regautocmddev114(&self) -> Regautocmddev114R {
        Regautocmddev114R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_115"]
    #[inline(always)]
    pub fn regautocmddev115(&self) -> Regautocmddev115R {
        Regautocmddev115R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_116"]
    #[inline(always)]
    pub fn regautocmddev116(&self) -> Regautocmddev116R {
        Regautocmddev116R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_117"]
    #[inline(always)]
    pub fn regautocmddev117(&self) -> Regautocmddev117R {
        Regautocmddev117R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_118"]
    #[inline(always)]
    pub fn regautocmddev118(&self) -> Regautocmddev118R {
        Regautocmddev118R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_119"]
    #[inline(always)]
    pub fn regautocmddev119(&self) -> Regautocmddev119R {
        Regautocmddev119R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_112"]
    #[inline(always)]
    pub fn regautocmddev112(&mut self) -> Regautocmddev112W<I3ccontrol078Spec> {
        Regautocmddev112W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_113"]
    #[inline(always)]
    pub fn regautocmddev113(&mut self) -> Regautocmddev113W<I3ccontrol078Spec> {
        Regautocmddev113W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_114"]
    #[inline(always)]
    pub fn regautocmddev114(&mut self) -> Regautocmddev114W<I3ccontrol078Spec> {
        Regautocmddev114W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_115"]
    #[inline(always)]
    pub fn regautocmddev115(&mut self) -> Regautocmddev115W<I3ccontrol078Spec> {
        Regautocmddev115W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_116"]
    #[inline(always)]
    pub fn regautocmddev116(&mut self) -> Regautocmddev116W<I3ccontrol078Spec> {
        Regautocmddev116W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_117"]
    #[inline(always)]
    pub fn regautocmddev117(&mut self) -> Regautocmddev117W<I3ccontrol078Spec> {
        Regautocmddev117W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_118"]
    #[inline(always)]
    pub fn regautocmddev118(&mut self) -> Regautocmddev118W<I3ccontrol078Spec> {
        Regautocmddev118W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_119"]
    #[inline(always)]
    pub fn regautocmddev119(&mut self) -> Regautocmddev119W<I3ccontrol078Spec> {
        Regautocmddev119W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_078\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol078Spec;
impl crate::RegisterSpec for I3ccontrol078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol078::R`](R) reader structure"]
impl crate::Readable for I3ccontrol078Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol078::W`](W) writer structure"]
impl crate::Writable for I3ccontrol078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL078 to value 0"]
impl crate::Resettable for I3ccontrol078Spec {}
