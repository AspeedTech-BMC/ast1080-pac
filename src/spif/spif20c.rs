#[doc = "Register `SPIF20C` reader"]
pub type R = crate::R<Spif20cSpec>;
#[doc = "Register `SPIF20C` writer"]
pub type W = crate::W<Spif20cSpec>;
#[doc = "Field `ADDRLBND03` reader - ADDR_LBND03"]
pub type Addrlbnd03R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND03` writer - ADDR_LBND03"]
pub type Addrlbnd03W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND03` reader - ADDR_UBND03"]
pub type Addrubnd03R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND03` writer - ADDR_UBND03"]
pub type Addrubnd03W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND03"]
    #[inline(always)]
    pub fn addrlbnd03(&self) -> Addrlbnd03R {
        Addrlbnd03R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND03"]
    #[inline(always)]
    pub fn addrubnd03(&self) -> Addrubnd03R {
        Addrubnd03R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND03"]
    #[inline(always)]
    pub fn addrlbnd03(&mut self) -> Addrlbnd03W<Spif20cSpec> {
        Addrlbnd03W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND03"]
    #[inline(always)]
    pub fn addrubnd03(&mut self) -> Addrubnd03W<Spif20cSpec> {
        Addrubnd03W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND03\n\nYou can [`read`](crate::Reg::read) this register and get [`spif20c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif20c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif20cSpec;
impl crate::RegisterSpec for Spif20cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif20c::R`](R) reader structure"]
impl crate::Readable for Spif20cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif20c::W`](W) writer structure"]
impl crate::Writable for Spif20cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF20C to value 0"]
impl crate::Resettable for Spif20cSpec {}
