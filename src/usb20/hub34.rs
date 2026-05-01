#[doc = "Register `HUB34` reader"]
pub type R = crate::R<Hub34Spec>;
#[doc = "Register `HUB34` writer"]
pub type W = crate::W<Hub34Spec>;
#[doc = "Field `BaseAddrOfEndpoint0INOUTDataBuf` reader - Base address of Endpoint 0 IN/OUT data buffer"]
pub type BaseAddrOfEndpoint0inoutdataBufR = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfEndpoint0INOUTDataBuf` writer - Base address of Endpoint 0 IN/OUT data buffer"]
pub type BaseAddrOfEndpoint0inoutdataBufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of Endpoint 0 IN/OUT data buffer"]
    #[inline(always)]
    pub fn base_addr_of_endpoint0inoutdata_buf(&self) -> BaseAddrOfEndpoint0inoutdataBufR {
        BaseAddrOfEndpoint0inoutdataBufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address of Endpoint 0 IN/OUT data buffer"]
    #[inline(always)]
    pub fn base_addr_of_endpoint0inoutdata_buf(
        &mut self,
    ) -> BaseAddrOfEndpoint0inoutdataBufW<Hub34Spec> {
        BaseAddrOfEndpoint0inoutdataBufW::new(self, 0)
    }
}
#[doc = "Base Address of Endpoint 0 IN/OUT Data Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub34Spec;
impl crate::RegisterSpec for Hub34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub34::R`](R) reader structure"]
impl crate::Readable for Hub34Spec {}
#[doc = "`write(|w| ..)` method takes [`hub34::W`](W) writer structure"]
impl crate::Writable for Hub34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB34 to value 0"]
impl crate::Resettable for Hub34Spec {}
