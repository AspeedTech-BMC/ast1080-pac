#[doc = "Register `SCU450` reader"]
pub type R = crate::R<Scu450Spec>;
#[doc = "Register `SCU450` writer"]
pub type W = crate::W<Scu450Spec>;
#[doc = "Field `SCUMUXIO160` reader - SCU_MUX_IO160"]
pub type Scumuxio160R = crate::FieldReader;
#[doc = "Field `SCUMUXIO160` writer - SCU_MUX_IO160"]
pub type Scumuxio160W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO161` reader - SCU_MUX_IO161"]
pub type Scumuxio161R = crate::FieldReader;
#[doc = "Field `SCUMUXIO161` writer - SCU_MUX_IO161"]
pub type Scumuxio161W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO162` reader - SCU_MUX_IO162"]
pub type Scumuxio162R = crate::FieldReader;
#[doc = "Field `SCUMUXIO162` writer - SCU_MUX_IO162"]
pub type Scumuxio162W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO163` reader - SCU_MUX_IO163"]
pub type Scumuxio163R = crate::FieldReader;
#[doc = "Field `SCUMUXIO163` writer - SCU_MUX_IO163"]
pub type Scumuxio163W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO164` reader - SCU_MUX_IO164"]
pub type Scumuxio164R = crate::FieldReader;
#[doc = "Field `SCUMUXIO164` writer - SCU_MUX_IO164"]
pub type Scumuxio164W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO165` reader - SCU_MUX_IO165"]
pub type Scumuxio165R = crate::FieldReader;
#[doc = "Field `SCUMUXIO165` writer - SCU_MUX_IO165"]
pub type Scumuxio165W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO166` reader - SCU_MUX_IO166"]
pub type Scumuxio166R = crate::FieldReader;
#[doc = "Field `SCUMUXIO166` writer - SCU_MUX_IO166"]
pub type Scumuxio166W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO167` reader - SCU_MUX_IO167"]
pub type Scumuxio167R = crate::FieldReader;
#[doc = "Field `SCUMUXIO167` writer - SCU_MUX_IO167"]
pub type Scumuxio167W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO160"]
    #[inline(always)]
    pub fn scumuxio160(&self) -> Scumuxio160R {
        Scumuxio160R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO161"]
    #[inline(always)]
    pub fn scumuxio161(&self) -> Scumuxio161R {
        Scumuxio161R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO162"]
    #[inline(always)]
    pub fn scumuxio162(&self) -> Scumuxio162R {
        Scumuxio162R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO163"]
    #[inline(always)]
    pub fn scumuxio163(&self) -> Scumuxio163R {
        Scumuxio163R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO164"]
    #[inline(always)]
    pub fn scumuxio164(&self) -> Scumuxio164R {
        Scumuxio164R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO165"]
    #[inline(always)]
    pub fn scumuxio165(&self) -> Scumuxio165R {
        Scumuxio165R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO166"]
    #[inline(always)]
    pub fn scumuxio166(&self) -> Scumuxio166R {
        Scumuxio166R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO167"]
    #[inline(always)]
    pub fn scumuxio167(&self) -> Scumuxio167R {
        Scumuxio167R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO160"]
    #[inline(always)]
    pub fn scumuxio160(&mut self) -> Scumuxio160W<Scu450Spec> {
        Scumuxio160W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO161"]
    #[inline(always)]
    pub fn scumuxio161(&mut self) -> Scumuxio161W<Scu450Spec> {
        Scumuxio161W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO162"]
    #[inline(always)]
    pub fn scumuxio162(&mut self) -> Scumuxio162W<Scu450Spec> {
        Scumuxio162W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO163"]
    #[inline(always)]
    pub fn scumuxio163(&mut self) -> Scumuxio163W<Scu450Spec> {
        Scumuxio163W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO164"]
    #[inline(always)]
    pub fn scumuxio164(&mut self) -> Scumuxio164W<Scu450Spec> {
        Scumuxio164W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO165"]
    #[inline(always)]
    pub fn scumuxio165(&mut self) -> Scumuxio165W<Scu450Spec> {
        Scumuxio165W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO166"]
    #[inline(always)]
    pub fn scumuxio166(&mut self) -> Scumuxio166W<Scu450Spec> {
        Scumuxio166W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO167"]
    #[inline(always)]
    pub fn scumuxio167(&mut self) -> Scumuxio167W<Scu450Spec> {
        Scumuxio167W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scu450::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu450::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu450Spec;
impl crate::RegisterSpec for Scu450Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu450::R`](R) reader structure"]
impl crate::Readable for Scu450Spec {}
#[doc = "`write(|w| ..)` method takes [`scu450::W`](W) writer structure"]
impl crate::Writable for Scu450Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU450 to value 0x1120_0000"]
impl crate::Resettable for Scu450Spec {
    const RESET_VALUE: u32 = 0x1120_0000;
}
