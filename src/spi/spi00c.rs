#[doc = "Register `SPI00C` reader"]
pub type R = crate::R<Spi00cSpec>;
#[doc = "Register `SPI00C` writer"]
pub type W = crate::W<Spi00cSpec>;
#[doc = "Field `DATABEB` reader - DATA_BEB"]
pub type DatabebR = crate::FieldReader;
#[doc = "Field `DATABEB` writer - DATA_BEB"]
pub type DatabebW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRBEB` reader - ADDR_BEB"]
pub type AddrbebR = crate::FieldReader;
#[doc = "Field `ADDRBEB` writer - ADDR_BEB"]
pub type AddrbebW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DATA_BEB"]
    #[inline(always)]
    pub fn databeb(&self) -> DatabebR {
        DatabebR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADDR_BEB"]
    #[inline(always)]
    pub fn addrbeb(&self) -> AddrbebR {
        AddrbebR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DATA_BEB"]
    #[inline(always)]
    pub fn databeb(&mut self) -> DatabebW<Spi00cSpec> {
        DatabebW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ADDR_BEB"]
    #[inline(always)]
    pub fn addrbeb(&mut self) -> AddrbebW<Spi00cSpec> {
        AddrbebW::new(self, 4)
    }
}
#[doc = "Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi00cSpec;
impl crate::RegisterSpec for Spi00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi00c::R`](R) reader structure"]
impl crate::Readable for Spi00cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi00c::W`](W) writer structure"]
impl crate::Writable for Spi00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI00C to value 0"]
impl crate::Resettable for Spi00cSpec {}
