#[doc = "Register `SPIF220` reader"]
pub type R = crate::R<Spif220Spec>;
#[doc = "Register `SPIF220` writer"]
pub type W = crate::W<Spif220Spec>;
#[doc = "Field `ADDRLBND08` reader - ADDR_LBND08"]
pub type Addrlbnd08R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND08` writer - ADDR_LBND08"]
pub type Addrlbnd08W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND08` reader - ADDR_UBND08"]
pub type Addrubnd08R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND08` writer - ADDR_UBND08"]
pub type Addrubnd08W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND08"]
    #[inline(always)]
    pub fn addrlbnd08(&self) -> Addrlbnd08R {
        Addrlbnd08R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND08"]
    #[inline(always)]
    pub fn addrubnd08(&self) -> Addrubnd08R {
        Addrubnd08R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND08"]
    #[inline(always)]
    pub fn addrlbnd08(&mut self) -> Addrlbnd08W<Spif220Spec> {
        Addrlbnd08W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND08"]
    #[inline(always)]
    pub fn addrubnd08(&mut self) -> Addrubnd08W<Spif220Spec> {
        Addrubnd08W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND08\n\nYou can [`read`](crate::Reg::read) this register and get [`spif220::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif220::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif220Spec;
impl crate::RegisterSpec for Spif220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif220::R`](R) reader structure"]
impl crate::Readable for Spif220Spec {}
#[doc = "`write(|w| ..)` method takes [`spif220::W`](W) writer structure"]
impl crate::Writable for Spif220Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF220 to value 0"]
impl crate::Resettable for Spif220Spec {}
