#[doc = "Register `SCU414` reader"]
pub type R = crate::R<Scu414Spec>;
#[doc = "Register `SCU414` writer"]
pub type W = crate::W<Scu414Spec>;
#[doc = "Field `SCUMUXIO040` reader - SCU_MUX_IO040"]
pub type Scumuxio040R = crate::FieldReader;
#[doc = "Field `SCUMUXIO040` writer - SCU_MUX_IO040"]
pub type Scumuxio040W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO041` reader - SCU_MUX_IO041"]
pub type Scumuxio041R = crate::FieldReader;
#[doc = "Field `SCUMUXIO041` writer - SCU_MUX_IO041"]
pub type Scumuxio041W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO042` reader - SCU_MUX_IO042"]
pub type Scumuxio042R = crate::FieldReader;
#[doc = "Field `SCUMUXIO042` writer - SCU_MUX_IO042"]
pub type Scumuxio042W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO043` reader - SCU_MUX_IO043"]
pub type Scumuxio043R = crate::FieldReader;
#[doc = "Field `SCUMUXIO043` writer - SCU_MUX_IO043"]
pub type Scumuxio043W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO044` reader - SCU_MUX_IO044"]
pub type Scumuxio044R = crate::FieldReader;
#[doc = "Field `SCUMUXIO044` writer - SCU_MUX_IO044"]
pub type Scumuxio044W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO045` reader - SCU_MUX_IO045"]
pub type Scumuxio045R = crate::FieldReader;
#[doc = "Field `SCUMUXIO045` writer - SCU_MUX_IO045"]
pub type Scumuxio045W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO046` reader - SCU_MUX_IO046"]
pub type Scumuxio046R = crate::FieldReader;
#[doc = "Field `SCUMUXIO046` writer - SCU_MUX_IO046"]
pub type Scumuxio046W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO047` reader - SCU_MUX_IO047"]
pub type Scumuxio047R = crate::FieldReader;
#[doc = "Field `SCUMUXIO047` writer - SCU_MUX_IO047"]
pub type Scumuxio047W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO040"]
    #[inline(always)]
    pub fn scumuxio040(&self) -> Scumuxio040R {
        Scumuxio040R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO041"]
    #[inline(always)]
    pub fn scumuxio041(&self) -> Scumuxio041R {
        Scumuxio041R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO042"]
    #[inline(always)]
    pub fn scumuxio042(&self) -> Scumuxio042R {
        Scumuxio042R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO043"]
    #[inline(always)]
    pub fn scumuxio043(&self) -> Scumuxio043R {
        Scumuxio043R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO044"]
    #[inline(always)]
    pub fn scumuxio044(&self) -> Scumuxio044R {
        Scumuxio044R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO045"]
    #[inline(always)]
    pub fn scumuxio045(&self) -> Scumuxio045R {
        Scumuxio045R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO046"]
    #[inline(always)]
    pub fn scumuxio046(&self) -> Scumuxio046R {
        Scumuxio046R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO047"]
    #[inline(always)]
    pub fn scumuxio047(&self) -> Scumuxio047R {
        Scumuxio047R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO040"]
    #[inline(always)]
    pub fn scumuxio040(&mut self) -> Scumuxio040W<Scu414Spec> {
        Scumuxio040W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO041"]
    #[inline(always)]
    pub fn scumuxio041(&mut self) -> Scumuxio041W<Scu414Spec> {
        Scumuxio041W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO042"]
    #[inline(always)]
    pub fn scumuxio042(&mut self) -> Scumuxio042W<Scu414Spec> {
        Scumuxio042W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO043"]
    #[inline(always)]
    pub fn scumuxio043(&mut self) -> Scumuxio043W<Scu414Spec> {
        Scumuxio043W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO044"]
    #[inline(always)]
    pub fn scumuxio044(&mut self) -> Scumuxio044W<Scu414Spec> {
        Scumuxio044W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO045"]
    #[inline(always)]
    pub fn scumuxio045(&mut self) -> Scumuxio045W<Scu414Spec> {
        Scumuxio045W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO046"]
    #[inline(always)]
    pub fn scumuxio046(&mut self) -> Scumuxio046W<Scu414Spec> {
        Scumuxio046W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO047"]
    #[inline(always)]
    pub fn scumuxio047(&mut self) -> Scumuxio047W<Scu414Spec> {
        Scumuxio047W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu414::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu414::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu414Spec;
impl crate::RegisterSpec for Scu414Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu414::R`](R) reader structure"]
impl crate::Readable for Scu414Spec {}
#[doc = "`write(|w| ..)` method takes [`scu414::W`](W) writer structure"]
impl crate::Writable for Scu414Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU414 to value 0x0011_0000"]
impl crate::Resettable for Scu414Spec {
    const RESET_VALUE: u32 = 0x0011_0000;
}
