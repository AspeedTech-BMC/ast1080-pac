#[doc = "Register `I3CCONTROL04C` reader"]
pub type R = crate::R<I3ccontrol04cSpec>;
#[doc = "Register `I3CCONTROL04C` writer"]
pub type W = crate::W<I3ccontrol04cSpec>;
#[doc = "Field `REGAUTOCMDDEV24` reader - REG_AUTOCMD_DEV_24"]
pub type Regautocmddev24R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV24` writer - REG_AUTOCMD_DEV_24"]
pub type Regautocmddev24W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV25` reader - REG_AUTOCMD_DEV_25"]
pub type Regautocmddev25R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV25` writer - REG_AUTOCMD_DEV_25"]
pub type Regautocmddev25W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV26` reader - REG_AUTOCMD_DEV_26"]
pub type Regautocmddev26R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV26` writer - REG_AUTOCMD_DEV_26"]
pub type Regautocmddev26W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV27` reader - REG_AUTOCMD_DEV_27"]
pub type Regautocmddev27R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV27` writer - REG_AUTOCMD_DEV_27"]
pub type Regautocmddev27W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV28` reader - REG_AUTOCMD_DEV_28"]
pub type Regautocmddev28R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV28` writer - REG_AUTOCMD_DEV_28"]
pub type Regautocmddev28W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV29` reader - REG_AUTOCMD_DEV_29"]
pub type Regautocmddev29R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV29` writer - REG_AUTOCMD_DEV_29"]
pub type Regautocmddev29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV30` reader - REG_AUTOCMD_DEV_30"]
pub type Regautocmddev30R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV30` writer - REG_AUTOCMD_DEV_30"]
pub type Regautocmddev30W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV31` reader - REG_AUTOCMD_DEV_31"]
pub type Regautocmddev31R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV31` writer - REG_AUTOCMD_DEV_31"]
pub type Regautocmddev31W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_24"]
    #[inline(always)]
    pub fn regautocmddev24(&self) -> Regautocmddev24R {
        Regautocmddev24R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_25"]
    #[inline(always)]
    pub fn regautocmddev25(&self) -> Regautocmddev25R {
        Regautocmddev25R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_26"]
    #[inline(always)]
    pub fn regautocmddev26(&self) -> Regautocmddev26R {
        Regautocmddev26R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_27"]
    #[inline(always)]
    pub fn regautocmddev27(&self) -> Regautocmddev27R {
        Regautocmddev27R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_28"]
    #[inline(always)]
    pub fn regautocmddev28(&self) -> Regautocmddev28R {
        Regautocmddev28R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_29"]
    #[inline(always)]
    pub fn regautocmddev29(&self) -> Regautocmddev29R {
        Regautocmddev29R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_30"]
    #[inline(always)]
    pub fn regautocmddev30(&self) -> Regautocmddev30R {
        Regautocmddev30R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_31"]
    #[inline(always)]
    pub fn regautocmddev31(&self) -> Regautocmddev31R {
        Regautocmddev31R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_24"]
    #[inline(always)]
    pub fn regautocmddev24(&mut self) -> Regautocmddev24W<I3ccontrol04cSpec> {
        Regautocmddev24W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_25"]
    #[inline(always)]
    pub fn regautocmddev25(&mut self) -> Regautocmddev25W<I3ccontrol04cSpec> {
        Regautocmddev25W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_26"]
    #[inline(always)]
    pub fn regautocmddev26(&mut self) -> Regautocmddev26W<I3ccontrol04cSpec> {
        Regautocmddev26W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_27"]
    #[inline(always)]
    pub fn regautocmddev27(&mut self) -> Regautocmddev27W<I3ccontrol04cSpec> {
        Regautocmddev27W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_28"]
    #[inline(always)]
    pub fn regautocmddev28(&mut self) -> Regautocmddev28W<I3ccontrol04cSpec> {
        Regautocmddev28W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_29"]
    #[inline(always)]
    pub fn regautocmddev29(&mut self) -> Regautocmddev29W<I3ccontrol04cSpec> {
        Regautocmddev29W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_30"]
    #[inline(always)]
    pub fn regautocmddev30(&mut self) -> Regautocmddev30W<I3ccontrol04cSpec> {
        Regautocmddev30W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_31"]
    #[inline(always)]
    pub fn regautocmddev31(&mut self) -> Regautocmddev31W<I3ccontrol04cSpec> {
        Regautocmddev31W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_04C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol04cSpec;
impl crate::RegisterSpec for I3ccontrol04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol04c::R`](R) reader structure"]
impl crate::Readable for I3ccontrol04cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol04c::W`](W) writer structure"]
impl crate::Writable for I3ccontrol04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL04C to value 0"]
impl crate::Resettable for I3ccontrol04cSpec {}
