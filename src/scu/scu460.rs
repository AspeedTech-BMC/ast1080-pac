#[doc = "Register `SCU460` reader"]
pub type R = crate::R<Scu460Spec>;
#[doc = "Register `SCU460` writer"]
pub type W = crate::W<Scu460Spec>;
#[doc = "Field `SCUMUXIO192` reader - SCU_MUX_IO192"]
pub type Scumuxio192R = crate::FieldReader;
#[doc = "Field `SCUMUXIO192` writer - SCU_MUX_IO192"]
pub type Scumuxio192W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO193` reader - SCU_MUX_IO193"]
pub type Scumuxio193R = crate::FieldReader;
#[doc = "Field `SCUMUXIO193` writer - SCU_MUX_IO193"]
pub type Scumuxio193W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO192"]
    #[inline(always)]
    pub fn scumuxio192(&self) -> Scumuxio192R {
        Scumuxio192R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO193"]
    #[inline(always)]
    pub fn scumuxio193(&self) -> Scumuxio193R {
        Scumuxio193R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO192"]
    #[inline(always)]
    pub fn scumuxio192(&mut self) -> Scumuxio192W<Scu460Spec> {
        Scumuxio192W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO193"]
    #[inline(always)]
    pub fn scumuxio193(&mut self) -> Scumuxio193W<Scu460Spec> {
        Scumuxio193W::new(self, 4)
    }
}
#[doc = "Multi-Function Pin Control \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu460::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu460::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu460Spec;
impl crate::RegisterSpec for Scu460Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu460::R`](R) reader structure"]
impl crate::Readable for Scu460Spec {}
#[doc = "`write(|w| ..)` method takes [`scu460::W`](W) writer structure"]
impl crate::Writable for Scu460Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU460 to value 0x10"]
impl crate::Resettable for Scu460Spec {
    const RESET_VALUE: u32 = 0x10;
}
