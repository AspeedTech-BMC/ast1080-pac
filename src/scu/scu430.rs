#[doc = "Register `SCU430` reader"]
pub type R = crate::R<Scu430Spec>;
#[doc = "Register `SCU430` writer"]
pub type W = crate::W<Scu430Spec>;
#[doc = "Field `SCUMUXIO096` reader - SCU_MUX_IO096"]
pub type Scumuxio096R = crate::FieldReader;
#[doc = "Field `SCUMUXIO096` writer - SCU_MUX_IO096"]
pub type Scumuxio096W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO097` reader - SCU_MUX_IO097"]
pub type Scumuxio097R = crate::FieldReader;
#[doc = "Field `SCUMUXIO097` writer - SCU_MUX_IO097"]
pub type Scumuxio097W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO098` reader - SCU_MUX_IO098"]
pub type Scumuxio098R = crate::FieldReader;
#[doc = "Field `SCUMUXIO098` writer - SCU_MUX_IO098"]
pub type Scumuxio098W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO099` reader - SCU_MUX_IO099"]
pub type Scumuxio099R = crate::FieldReader;
#[doc = "Field `SCUMUXIO099` writer - SCU_MUX_IO099"]
pub type Scumuxio099W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO100` reader - SCU_MUX_IO100"]
pub type Scumuxio100R = crate::FieldReader;
#[doc = "Field `SCUMUXIO100` writer - SCU_MUX_IO100"]
pub type Scumuxio100W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO101` reader - SCU_MUX_IO101"]
pub type Scumuxio101R = crate::FieldReader;
#[doc = "Field `SCUMUXIO101` writer - SCU_MUX_IO101"]
pub type Scumuxio101W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO102` reader - SCU_MUX_IO102"]
pub type Scumuxio102R = crate::FieldReader;
#[doc = "Field `SCUMUXIO102` writer - SCU_MUX_IO102"]
pub type Scumuxio102W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO103` reader - SCU_MUX_IO103"]
pub type Scumuxio103R = crate::FieldReader;
#[doc = "Field `SCUMUXIO103` writer - SCU_MUX_IO103"]
pub type Scumuxio103W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO096"]
    #[inline(always)]
    pub fn scumuxio096(&self) -> Scumuxio096R {
        Scumuxio096R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO097"]
    #[inline(always)]
    pub fn scumuxio097(&self) -> Scumuxio097R {
        Scumuxio097R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO098"]
    #[inline(always)]
    pub fn scumuxio098(&self) -> Scumuxio098R {
        Scumuxio098R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO099"]
    #[inline(always)]
    pub fn scumuxio099(&self) -> Scumuxio099R {
        Scumuxio099R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO100"]
    #[inline(always)]
    pub fn scumuxio100(&self) -> Scumuxio100R {
        Scumuxio100R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO101"]
    #[inline(always)]
    pub fn scumuxio101(&self) -> Scumuxio101R {
        Scumuxio101R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO102"]
    #[inline(always)]
    pub fn scumuxio102(&self) -> Scumuxio102R {
        Scumuxio102R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO103"]
    #[inline(always)]
    pub fn scumuxio103(&self) -> Scumuxio103R {
        Scumuxio103R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO096"]
    #[inline(always)]
    pub fn scumuxio096(&mut self) -> Scumuxio096W<Scu430Spec> {
        Scumuxio096W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO097"]
    #[inline(always)]
    pub fn scumuxio097(&mut self) -> Scumuxio097W<Scu430Spec> {
        Scumuxio097W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO098"]
    #[inline(always)]
    pub fn scumuxio098(&mut self) -> Scumuxio098W<Scu430Spec> {
        Scumuxio098W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO099"]
    #[inline(always)]
    pub fn scumuxio099(&mut self) -> Scumuxio099W<Scu430Spec> {
        Scumuxio099W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO100"]
    #[inline(always)]
    pub fn scumuxio100(&mut self) -> Scumuxio100W<Scu430Spec> {
        Scumuxio100W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO101"]
    #[inline(always)]
    pub fn scumuxio101(&mut self) -> Scumuxio101W<Scu430Spec> {
        Scumuxio101W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO102"]
    #[inline(always)]
    pub fn scumuxio102(&mut self) -> Scumuxio102W<Scu430Spec> {
        Scumuxio102W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO103"]
    #[inline(always)]
    pub fn scumuxio103(&mut self) -> Scumuxio103W<Scu430Spec> {
        Scumuxio103W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu430::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu430::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu430Spec;
impl crate::RegisterSpec for Scu430Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu430::R`](R) reader structure"]
impl crate::Readable for Scu430Spec {}
#[doc = "`write(|w| ..)` method takes [`scu430::W`](W) writer structure"]
impl crate::Writable for Scu430Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU430 to value 0"]
impl crate::Resettable for Scu430Spec {}
