#[doc = "Register `SPIF238` reader"]
pub type R = crate::R<Spif238Spec>;
#[doc = "Register `SPIF238` writer"]
pub type W = crate::W<Spif238Spec>;
#[doc = "Field `ADDRLBND14` reader - ADDR_LBND14"]
pub type Addrlbnd14R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND14` writer - ADDR_LBND14"]
pub type Addrlbnd14W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND14` reader - ADDR_UBND14"]
pub type Addrubnd14R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND14` writer - ADDR_UBND14"]
pub type Addrubnd14W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND14"]
    #[inline(always)]
    pub fn addrlbnd14(&self) -> Addrlbnd14R {
        Addrlbnd14R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND14"]
    #[inline(always)]
    pub fn addrubnd14(&self) -> Addrubnd14R {
        Addrubnd14R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND14"]
    #[inline(always)]
    pub fn addrlbnd14(&mut self) -> Addrlbnd14W<Spif238Spec> {
        Addrlbnd14W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND14"]
    #[inline(always)]
    pub fn addrubnd14(&mut self) -> Addrubnd14W<Spif238Spec> {
        Addrubnd14W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND14\n\nYou can [`read`](crate::Reg::read) this register and get [`spif238::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif238::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif238Spec;
impl crate::RegisterSpec for Spif238Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif238::R`](R) reader structure"]
impl crate::Readable for Spif238Spec {}
#[doc = "`write(|w| ..)` method takes [`spif238::W`](W) writer structure"]
impl crate::Writable for Spif238Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF238 to value 0"]
impl crate::Resettable for Spif238Spec {}
