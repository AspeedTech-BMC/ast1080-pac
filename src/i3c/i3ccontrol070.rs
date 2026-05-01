#[doc = "Register `I3CCONTROL070` reader"]
pub type R = crate::R<I3ccontrol070Spec>;
#[doc = "Register `I3CCONTROL070` writer"]
pub type W = crate::W<I3ccontrol070Spec>;
#[doc = "Field `REGAUTOCMDDEV96` reader - REG_AUTOCMD_DEV_96"]
pub type Regautocmddev96R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV96` writer - REG_AUTOCMD_DEV_96"]
pub type Regautocmddev96W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV97` reader - REG_AUTOCMD_DEV_97"]
pub type Regautocmddev97R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV97` writer - REG_AUTOCMD_DEV_97"]
pub type Regautocmddev97W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV98` reader - REG_AUTOCMD_DEV_98"]
pub type Regautocmddev98R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV98` writer - REG_AUTOCMD_DEV_98"]
pub type Regautocmddev98W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV99` reader - REG_AUTOCMD_DEV_99"]
pub type Regautocmddev99R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV99` writer - REG_AUTOCMD_DEV_99"]
pub type Regautocmddev99W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV100` reader - REG_AUTOCMD_DEV_100"]
pub type Regautocmddev100R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV100` writer - REG_AUTOCMD_DEV_100"]
pub type Regautocmddev100W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV101` reader - REG_AUTOCMD_DEV_101"]
pub type Regautocmddev101R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV101` writer - REG_AUTOCMD_DEV_101"]
pub type Regautocmddev101W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV102` reader - REG_AUTOCMD_DEV_102"]
pub type Regautocmddev102R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV102` writer - REG_AUTOCMD_DEV_102"]
pub type Regautocmddev102W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV103` reader - REG_AUTOCMD_DEV_103"]
pub type Regautocmddev103R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV103` writer - REG_AUTOCMD_DEV_103"]
pub type Regautocmddev103W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_96"]
    #[inline(always)]
    pub fn regautocmddev96(&self) -> Regautocmddev96R {
        Regautocmddev96R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_97"]
    #[inline(always)]
    pub fn regautocmddev97(&self) -> Regautocmddev97R {
        Regautocmddev97R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_98"]
    #[inline(always)]
    pub fn regautocmddev98(&self) -> Regautocmddev98R {
        Regautocmddev98R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_99"]
    #[inline(always)]
    pub fn regautocmddev99(&self) -> Regautocmddev99R {
        Regautocmddev99R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_100"]
    #[inline(always)]
    pub fn regautocmddev100(&self) -> Regautocmddev100R {
        Regautocmddev100R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_101"]
    #[inline(always)]
    pub fn regautocmddev101(&self) -> Regautocmddev101R {
        Regautocmddev101R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_102"]
    #[inline(always)]
    pub fn regautocmddev102(&self) -> Regautocmddev102R {
        Regautocmddev102R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_103"]
    #[inline(always)]
    pub fn regautocmddev103(&self) -> Regautocmddev103R {
        Regautocmddev103R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_96"]
    #[inline(always)]
    pub fn regautocmddev96(&mut self) -> Regautocmddev96W<I3ccontrol070Spec> {
        Regautocmddev96W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_97"]
    #[inline(always)]
    pub fn regautocmddev97(&mut self) -> Regautocmddev97W<I3ccontrol070Spec> {
        Regautocmddev97W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_98"]
    #[inline(always)]
    pub fn regautocmddev98(&mut self) -> Regautocmddev98W<I3ccontrol070Spec> {
        Regautocmddev98W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_99"]
    #[inline(always)]
    pub fn regautocmddev99(&mut self) -> Regautocmddev99W<I3ccontrol070Spec> {
        Regautocmddev99W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_100"]
    #[inline(always)]
    pub fn regautocmddev100(&mut self) -> Regautocmddev100W<I3ccontrol070Spec> {
        Regautocmddev100W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_101"]
    #[inline(always)]
    pub fn regautocmddev101(&mut self) -> Regautocmddev101W<I3ccontrol070Spec> {
        Regautocmddev101W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_102"]
    #[inline(always)]
    pub fn regautocmddev102(&mut self) -> Regautocmddev102W<I3ccontrol070Spec> {
        Regautocmddev102W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_103"]
    #[inline(always)]
    pub fn regautocmddev103(&mut self) -> Regautocmddev103W<I3ccontrol070Spec> {
        Regautocmddev103W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_070\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol070Spec;
impl crate::RegisterSpec for I3ccontrol070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol070::R`](R) reader structure"]
impl crate::Readable for I3ccontrol070Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol070::W`](W) writer structure"]
impl crate::Writable for I3ccontrol070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL070 to value 0"]
impl crate::Resettable for I3ccontrol070Spec {}
