#[doc = "Register `I3CCONTROL05C` reader"]
pub type R = crate::R<I3ccontrol05cSpec>;
#[doc = "Register `I3CCONTROL05C` writer"]
pub type W = crate::W<I3ccontrol05cSpec>;
#[doc = "Field `REGAUTOCMDDEV56` reader - REG_AUTOCMD_DEV_56"]
pub type Regautocmddev56R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV56` writer - REG_AUTOCMD_DEV_56"]
pub type Regautocmddev56W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV57` reader - REG_AUTOCMD_DEV_57"]
pub type Regautocmddev57R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV57` writer - REG_AUTOCMD_DEV_57"]
pub type Regautocmddev57W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV58` reader - REG_AUTOCMD_DEV_58"]
pub type Regautocmddev58R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV58` writer - REG_AUTOCMD_DEV_58"]
pub type Regautocmddev58W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV59` reader - REG_AUTOCMD_DEV_59"]
pub type Regautocmddev59R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV59` writer - REG_AUTOCMD_DEV_59"]
pub type Regautocmddev59W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV60` reader - REG_AUTOCMD_DEV_60"]
pub type Regautocmddev60R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV60` writer - REG_AUTOCMD_DEV_60"]
pub type Regautocmddev60W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV61` reader - REG_AUTOCMD_DEV_61"]
pub type Regautocmddev61R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV61` writer - REG_AUTOCMD_DEV_61"]
pub type Regautocmddev61W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV62` reader - REG_AUTOCMD_DEV_62"]
pub type Regautocmddev62R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV62` writer - REG_AUTOCMD_DEV_62"]
pub type Regautocmddev62W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV63` reader - REG_AUTOCMD_DEV_63"]
pub type Regautocmddev63R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV63` writer - REG_AUTOCMD_DEV_63"]
pub type Regautocmddev63W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_56"]
    #[inline(always)]
    pub fn regautocmddev56(&self) -> Regautocmddev56R {
        Regautocmddev56R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_57"]
    #[inline(always)]
    pub fn regautocmddev57(&self) -> Regautocmddev57R {
        Regautocmddev57R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_58"]
    #[inline(always)]
    pub fn regautocmddev58(&self) -> Regautocmddev58R {
        Regautocmddev58R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_59"]
    #[inline(always)]
    pub fn regautocmddev59(&self) -> Regautocmddev59R {
        Regautocmddev59R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_60"]
    #[inline(always)]
    pub fn regautocmddev60(&self) -> Regautocmddev60R {
        Regautocmddev60R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_61"]
    #[inline(always)]
    pub fn regautocmddev61(&self) -> Regautocmddev61R {
        Regautocmddev61R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_62"]
    #[inline(always)]
    pub fn regautocmddev62(&self) -> Regautocmddev62R {
        Regautocmddev62R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_63"]
    #[inline(always)]
    pub fn regautocmddev63(&self) -> Regautocmddev63R {
        Regautocmddev63R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_56"]
    #[inline(always)]
    pub fn regautocmddev56(&mut self) -> Regautocmddev56W<I3ccontrol05cSpec> {
        Regautocmddev56W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_57"]
    #[inline(always)]
    pub fn regautocmddev57(&mut self) -> Regautocmddev57W<I3ccontrol05cSpec> {
        Regautocmddev57W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_58"]
    #[inline(always)]
    pub fn regautocmddev58(&mut self) -> Regautocmddev58W<I3ccontrol05cSpec> {
        Regautocmddev58W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_59"]
    #[inline(always)]
    pub fn regautocmddev59(&mut self) -> Regautocmddev59W<I3ccontrol05cSpec> {
        Regautocmddev59W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_60"]
    #[inline(always)]
    pub fn regautocmddev60(&mut self) -> Regautocmddev60W<I3ccontrol05cSpec> {
        Regautocmddev60W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_61"]
    #[inline(always)]
    pub fn regautocmddev61(&mut self) -> Regautocmddev61W<I3ccontrol05cSpec> {
        Regautocmddev61W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_62"]
    #[inline(always)]
    pub fn regautocmddev62(&mut self) -> Regautocmddev62W<I3ccontrol05cSpec> {
        Regautocmddev62W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_63"]
    #[inline(always)]
    pub fn regautocmddev63(&mut self) -> Regautocmddev63W<I3ccontrol05cSpec> {
        Regautocmddev63W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_05C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol05cSpec;
impl crate::RegisterSpec for I3ccontrol05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol05c::R`](R) reader structure"]
impl crate::Readable for I3ccontrol05cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol05c::W`](W) writer structure"]
impl crate::Writable for I3ccontrol05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL05C to value 0"]
impl crate::Resettable for I3ccontrol05cSpec {}
