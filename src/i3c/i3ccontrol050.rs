#[doc = "Register `I3CCONTROL050` reader"]
pub type R = crate::R<I3ccontrol050Spec>;
#[doc = "Register `I3CCONTROL050` writer"]
pub type W = crate::W<I3ccontrol050Spec>;
#[doc = "Field `REGAUTOCMDDEV32` reader - REG_AUTOCMD_DEV_32"]
pub type Regautocmddev32R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV32` writer - REG_AUTOCMD_DEV_32"]
pub type Regautocmddev32W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV33` reader - REG_AUTOCMD_DEV_33"]
pub type Regautocmddev33R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV33` writer - REG_AUTOCMD_DEV_33"]
pub type Regautocmddev33W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV34` reader - REG_AUTOCMD_DEV_34"]
pub type Regautocmddev34R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV34` writer - REG_AUTOCMD_DEV_34"]
pub type Regautocmddev34W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV35` reader - REG_AUTOCMD_DEV_35"]
pub type Regautocmddev35R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV35` writer - REG_AUTOCMD_DEV_35"]
pub type Regautocmddev35W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV36` reader - REG_AUTOCMD_DEV_36"]
pub type Regautocmddev36R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV36` writer - REG_AUTOCMD_DEV_36"]
pub type Regautocmddev36W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV37` reader - REG_AUTOCMD_DEV_37"]
pub type Regautocmddev37R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV37` writer - REG_AUTOCMD_DEV_37"]
pub type Regautocmddev37W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV38` reader - REG_AUTOCMD_DEV_38"]
pub type Regautocmddev38R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV38` writer - REG_AUTOCMD_DEV_38"]
pub type Regautocmddev38W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV39` reader - REG_AUTOCMD_DEV_39"]
pub type Regautocmddev39R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV39` writer - REG_AUTOCMD_DEV_39"]
pub type Regautocmddev39W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_32"]
    #[inline(always)]
    pub fn regautocmddev32(&self) -> Regautocmddev32R {
        Regautocmddev32R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_33"]
    #[inline(always)]
    pub fn regautocmddev33(&self) -> Regautocmddev33R {
        Regautocmddev33R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_34"]
    #[inline(always)]
    pub fn regautocmddev34(&self) -> Regautocmddev34R {
        Regautocmddev34R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_35"]
    #[inline(always)]
    pub fn regautocmddev35(&self) -> Regautocmddev35R {
        Regautocmddev35R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_36"]
    #[inline(always)]
    pub fn regautocmddev36(&self) -> Regautocmddev36R {
        Regautocmddev36R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_37"]
    #[inline(always)]
    pub fn regautocmddev37(&self) -> Regautocmddev37R {
        Regautocmddev37R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_38"]
    #[inline(always)]
    pub fn regautocmddev38(&self) -> Regautocmddev38R {
        Regautocmddev38R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_39"]
    #[inline(always)]
    pub fn regautocmddev39(&self) -> Regautocmddev39R {
        Regautocmddev39R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_32"]
    #[inline(always)]
    pub fn regautocmddev32(&mut self) -> Regautocmddev32W<I3ccontrol050Spec> {
        Regautocmddev32W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_33"]
    #[inline(always)]
    pub fn regautocmddev33(&mut self) -> Regautocmddev33W<I3ccontrol050Spec> {
        Regautocmddev33W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_34"]
    #[inline(always)]
    pub fn regautocmddev34(&mut self) -> Regautocmddev34W<I3ccontrol050Spec> {
        Regautocmddev34W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_35"]
    #[inline(always)]
    pub fn regautocmddev35(&mut self) -> Regautocmddev35W<I3ccontrol050Spec> {
        Regautocmddev35W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_36"]
    #[inline(always)]
    pub fn regautocmddev36(&mut self) -> Regautocmddev36W<I3ccontrol050Spec> {
        Regautocmddev36W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_37"]
    #[inline(always)]
    pub fn regautocmddev37(&mut self) -> Regautocmddev37W<I3ccontrol050Spec> {
        Regautocmddev37W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_38"]
    #[inline(always)]
    pub fn regautocmddev38(&mut self) -> Regautocmddev38W<I3ccontrol050Spec> {
        Regautocmddev38W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_39"]
    #[inline(always)]
    pub fn regautocmddev39(&mut self) -> Regautocmddev39W<I3ccontrol050Spec> {
        Regautocmddev39W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_050\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol050Spec;
impl crate::RegisterSpec for I3ccontrol050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol050::R`](R) reader structure"]
impl crate::Readable for I3ccontrol050Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol050::W`](W) writer structure"]
impl crate::Writable for I3ccontrol050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL050 to value 0"]
impl crate::Resettable for I3ccontrol050Spec {}
