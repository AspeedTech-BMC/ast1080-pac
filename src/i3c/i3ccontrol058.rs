#[doc = "Register `I3CCONTROL058` reader"]
pub type R = crate::R<I3ccontrol058Spec>;
#[doc = "Register `I3CCONTROL058` writer"]
pub type W = crate::W<I3ccontrol058Spec>;
#[doc = "Field `REGAUTOCMDDEV48` reader - REG_AUTOCMD_DEV_48"]
pub type Regautocmddev48R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV48` writer - REG_AUTOCMD_DEV_48"]
pub type Regautocmddev48W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV49` reader - REG_AUTOCMD_DEV_49"]
pub type Regautocmddev49R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV49` writer - REG_AUTOCMD_DEV_49"]
pub type Regautocmddev49W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV50` reader - REG_AUTOCMD_DEV_50"]
pub type Regautocmddev50R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV50` writer - REG_AUTOCMD_DEV_50"]
pub type Regautocmddev50W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV51` reader - REG_AUTOCMD_DEV_51"]
pub type Regautocmddev51R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV51` writer - REG_AUTOCMD_DEV_51"]
pub type Regautocmddev51W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV52` reader - REG_AUTOCMD_DEV_52"]
pub type Regautocmddev52R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV52` writer - REG_AUTOCMD_DEV_52"]
pub type Regautocmddev52W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV53` reader - REG_AUTOCMD_DEV_53"]
pub type Regautocmddev53R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV53` writer - REG_AUTOCMD_DEV_53"]
pub type Regautocmddev53W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV54` reader - REG_AUTOCMD_DEV_54"]
pub type Regautocmddev54R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV54` writer - REG_AUTOCMD_DEV_54"]
pub type Regautocmddev54W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV55` reader - REG_AUTOCMD_DEV_55"]
pub type Regautocmddev55R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV55` writer - REG_AUTOCMD_DEV_55"]
pub type Regautocmddev55W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_48"]
    #[inline(always)]
    pub fn regautocmddev48(&self) -> Regautocmddev48R {
        Regautocmddev48R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_49"]
    #[inline(always)]
    pub fn regautocmddev49(&self) -> Regautocmddev49R {
        Regautocmddev49R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_50"]
    #[inline(always)]
    pub fn regautocmddev50(&self) -> Regautocmddev50R {
        Regautocmddev50R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_51"]
    #[inline(always)]
    pub fn regautocmddev51(&self) -> Regautocmddev51R {
        Regautocmddev51R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_52"]
    #[inline(always)]
    pub fn regautocmddev52(&self) -> Regautocmddev52R {
        Regautocmddev52R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_53"]
    #[inline(always)]
    pub fn regautocmddev53(&self) -> Regautocmddev53R {
        Regautocmddev53R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_54"]
    #[inline(always)]
    pub fn regautocmddev54(&self) -> Regautocmddev54R {
        Regautocmddev54R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_55"]
    #[inline(always)]
    pub fn regautocmddev55(&self) -> Regautocmddev55R {
        Regautocmddev55R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_48"]
    #[inline(always)]
    pub fn regautocmddev48(&mut self) -> Regautocmddev48W<I3ccontrol058Spec> {
        Regautocmddev48W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_49"]
    #[inline(always)]
    pub fn regautocmddev49(&mut self) -> Regautocmddev49W<I3ccontrol058Spec> {
        Regautocmddev49W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_50"]
    #[inline(always)]
    pub fn regautocmddev50(&mut self) -> Regautocmddev50W<I3ccontrol058Spec> {
        Regautocmddev50W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_51"]
    #[inline(always)]
    pub fn regautocmddev51(&mut self) -> Regautocmddev51W<I3ccontrol058Spec> {
        Regautocmddev51W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_52"]
    #[inline(always)]
    pub fn regautocmddev52(&mut self) -> Regautocmddev52W<I3ccontrol058Spec> {
        Regautocmddev52W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_53"]
    #[inline(always)]
    pub fn regautocmddev53(&mut self) -> Regautocmddev53W<I3ccontrol058Spec> {
        Regautocmddev53W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_54"]
    #[inline(always)]
    pub fn regautocmddev54(&mut self) -> Regautocmddev54W<I3ccontrol058Spec> {
        Regautocmddev54W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_55"]
    #[inline(always)]
    pub fn regautocmddev55(&mut self) -> Regautocmddev55W<I3ccontrol058Spec> {
        Regautocmddev55W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_058\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol058Spec;
impl crate::RegisterSpec for I3ccontrol058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol058::R`](R) reader structure"]
impl crate::Readable for I3ccontrol058Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol058::W`](W) writer structure"]
impl crate::Writable for I3ccontrol058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL058 to value 0"]
impl crate::Resettable for I3ccontrol058Spec {}
