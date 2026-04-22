#[doc = "Register `SPIF004` reader"]
pub type R = crate::R<Spif004Spec>;
#[doc = "Register `SPIF004` writer"]
pub type W = crate::W<Spif004Spec>;
#[doc = "Field `INTSTS` reader - INT_STS"]
pub type IntstsR = crate::FieldReader<u16>;
#[doc = "Field `INTSTS` writer - INT_STS"]
pub type IntstsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INTEN` reader - INT_EN"]
pub type IntenR = crate::FieldReader<u16>;
#[doc = "Field `INTEN` writer - INT_EN"]
pub type IntenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - INT_STS"]
    #[inline(always)]
    pub fn intsts(&self) -> IntstsR {
        IntstsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - INT_EN"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - INT_STS"]
    #[inline(always)]
    pub fn intsts(&mut self) -> IntstsW<Spif004Spec> {
        IntstsW::new(self, 0)
    }
    #[doc = "Bits 16:31 - INT_EN"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<Spif004Spec> {
        IntenW::new(self, 16)
    }
}
#[doc = "SPIF\\_IRQ\n\nYou can [`read`](crate::Reg::read) this register and get [`spif004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif004Spec;
impl crate::RegisterSpec for Spif004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif004::R`](R) reader structure"]
impl crate::Readable for Spif004Spec {}
#[doc = "`write(|w| ..)` method takes [`spif004::W`](W) writer structure"]
impl crate::Writable for Spif004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF004 to value 0"]
impl crate::Resettable for Spif004Spec {}
